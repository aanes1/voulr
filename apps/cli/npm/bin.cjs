const fs = require("fs")
const os = require("os")
const path = require("path")
const tar = require("tar")
const { Readable } = require("stream")
const { version } = require("./package.json")
const { spawnSync } = require("child_process")

const platform = os.platform()
const arch = os.arch()

const filename = platform === "win32" ? "voulr.exe" : "voulr"
const bin = path.join(__dirname, filename)
const exists = fs.existsSync(bin)

function getTarget() {
	if (platform === "darwin" && arch === "arm64") {
		return "macos-silicon"
	} else if (platform === "darwin" && arch === "x64") {
		return "macos-intel"
	} else if (platform === "linux" && arch === "x64") {
		return "linux"
	} else if (platform === "win32" && arch === "x64") {
		return "windows"
	}
	throw new Error(`unsupported platform: ${platform} ${arch}`)
}

async function install() {
	if (exists) return

	const target = getTarget()
	const url = `https://github.com/aanes1/voulr/releases/download/v${version}/voulr-${target}.tar.gz`

	const res = await fetch(url)
	if (!res.ok) {
		throw new Error(`failed to fetch release: ${res.statusText}`)
	}

	const stream = Readable.fromWeb(res.body)
	const extractStream = tar.x({
		C: __dirname,
		strip: 1
	})

	await new Promise((resolve, reject) => {
		stream.pipe(extractStream).on("finish", resolve).on("error", reject)
	})

	if (platform !== "win32") {
		await fs.promises.chmod(bin, 0o755)
	}
}

async function run() {
	if (!exists) await install()

	const args = process.argv.slice(2)
	const child = spawnSync(bin, args, {
		cwd: process.cwd(),
		stdio: "inherit"
	})
	if (child.error) {
		console.error(child.error)
		process.exit(1)
	}
	process.exit(child.status)
}

module.exports = { install, run }

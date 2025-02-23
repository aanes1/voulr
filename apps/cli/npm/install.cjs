const fs = require("fs")
const os = require("os")
const path = require("path")
const tar = require("tar")
const { Readable } = require("stream")
const { version } = require("./package.json")

function getTarget() {
	const platform = os.platform()
	const arch = os.arch()

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
	const voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const targetExecutablePath = path.join(__dirname, voulrFileName)

	// binary already installed
	if (fs.existsSync(targetExecutablePath)) {
		return
	}

	const target = getTarget()
	const url = `https://github.com/aanes1/voulr/releases/download/${version}/voulr-${target}.tar.gz`

	const res = await fetch(url)
	if (!res.ok) {
		console.error(`error fetching release: ${res.statusText}`)
		process.exit(1)
	}

	const sink = Readable.fromWeb(res.body).pipe(tar.x({ C: __dirname }))
	return new Promise((resolve) => {
		sink.on("finish", () => resolve())
		sink.on("error", (err) => {
			console.error(`Error fetching release: ${err.message}`)
			process.exit(1)
		})
	})
}

install()

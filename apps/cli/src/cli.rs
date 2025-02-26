use crate::create::{create, CreateArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scaffold a new Voulr project", alias = "new")]
    Create(CreateArgs),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(cargs) => create(cargs)?,
    }

    Ok(())
}

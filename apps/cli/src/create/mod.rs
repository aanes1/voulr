use crate::utils::get_rcfg;
use anyhow::Result;
use clap::Args;
use framework::Framework;
use location::Location;

mod framework;
mod location;

#[derive(Args)]
pub struct CreateArgs {
    pub name: Option<String>,
    #[arg(long)]
    pub framework: Option<String>,
}

pub fn create(cargs: &CreateArgs) -> Result<()> {
    let rcfg = get_rcfg();

    let location = Location::from(&cargs.name, &rcfg)?;
    let framework = Framework::from(&cargs.framework, &rcfg)?;

    Ok(())
}

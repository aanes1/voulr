use crate::utils::get_rcfg;
use anyhow::Result;
use clap::Args;
use location::Location;

mod location;

#[derive(Args)]
pub struct CreateArgs {
    pub name: Option<String>,
}

pub fn create(cargs: &CreateArgs) -> Result<()> {
    let rcfg = get_rcfg();

    let location = Location::from(&cargs.name, &rcfg)?;

    Ok(())
}

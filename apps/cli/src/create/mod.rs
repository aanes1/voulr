use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct CreateArgs {
    pub name: Option<String>,
}

pub fn create(cargs: &CreateArgs) -> Result<()> {
    println!("name: {:?}", cargs.name);
    Ok(())
}

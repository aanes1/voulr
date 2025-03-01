use anyhow::Result;
use inquire::{ui::RenderConfig, Select};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(EnumIter, Debug, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Framework {
    Axum,
    Rocket,
    Actix,
}

impl Framework {
    pub fn from(cargs_fw: &Option<String>, rcfg: &RenderConfig) -> Result<Self> {
        let framework = match cargs_fw {
            Some(text) => Framework::from_str(text)?,
            None => Self::prompt(rcfg)?,
        };
        Ok(framework)
    }

    fn prompt(rcfg: &RenderConfig) -> Result<Framework> {
        let framework = Select::new("Framework", Framework::iter().collect())
            .with_render_config(*rcfg)
            .prompt()?;
        Ok(framework)
    }
}

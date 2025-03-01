use anyhow::{bail, Result};
use inquire::{ui::RenderConfig, validator::Validation, Confirm, Text};
use std::{ffi::OsStr, path::PathBuf};

const DEFAULT_NAME: &str = "my-server";
const MIN_NAME_LEN: usize = 1;
const MAX_NAME_LEN: usize = 64;
const VALID_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
];

pub struct Location {
    name: String,
    path: PathBuf,
}

impl Location {
    pub fn from(cargs_name: &Option<String>, rcfg: &RenderConfig) -> Result<Self> {
        let text = match cargs_name {
            Some(text) => text.to_string(),
            None => Self::prompt(rcfg)?,
        };

        let path = PathBuf::from(&text);
        let name = Self::get_file_name(&path);
        Self::validate(&name, &path)?;

        let location = Self { name, path };
        Ok(location)
    }

    fn prompt(rcfg: &RenderConfig) -> Result<String> {
        let text = Text::new("Project name")
            .with_default(DEFAULT_NAME)
            .with_render_config(*rcfg)
            .with_validator(|text: &str| {
                let path = PathBuf::from(&text);
                let name = Self::get_file_name(&path);
                match Self::validate(&name, &path) {
                    Ok(_) => Ok(Validation::Valid),
                    Err(e) => Ok(Validation::Invalid(e.into())),
                }
            })
            .prompt()?;
        Ok(text)
    }

    fn get_file_name(path: &PathBuf) -> String {
        path.file_name()
            .unwrap_or(OsStr::new(path))
            .to_string_lossy()
            .to_string()
    }

    fn validate(name: &str, path: &PathBuf) -> Result<()> {
        if name.len() <= MIN_NAME_LEN {
            bail!("Project name must be at least {} characters", MIN_NAME_LEN);
        }
        if name.len() >= MAX_NAME_LEN {
            bail!("Project name must be at most {} characters", MAX_NAME_LEN);
        }
        if !name.chars().all(|c| VALID_CHARS.contains(&c)) {
            bail!("Project name must only contain lowercase alphanumeric characters, dashes, and underscores");
        }
        Ok(())
    }
}

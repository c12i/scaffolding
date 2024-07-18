use crate::error::{ScaffoldError, ScaffoldResult};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ExampleType {
    HelloWorld,
    Forum,
}

impl ExampleType {
    pub fn choose() -> ScaffoldResult<Self> {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose example:")
            .item(ExampleType::HelloWorld)
            .item(ExampleType::Forum)
            .default(0)
            .interact()?;

        match selection {
            0 => Ok(ExampleType::HelloWorld),
            _ => Ok(ExampleType::Forum),
        }
    }
}

impl std::fmt::Display for ExampleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ExampleType::HelloWorld => "hello-world",
            ExampleType::Forum => "forum",
        };
        write!(f, "{str}")
    }
}

impl FromStr for ExampleType {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<Self> {
        match s {
            "hello-world" => Ok(ExampleType::HelloWorld),
            "forum" => Ok(ExampleType::Forum),
            _ => Err(ScaffoldError::InvalidExampleType(
                s.to_string(),
                "hello-world, forum".to_string(),
            )),
        }
    }
}

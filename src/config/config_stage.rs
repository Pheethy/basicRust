use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Local => write!(f, "Local"),
            Stage::Development => write!(f, "Development"),
            Stage::Production => write!(f, "Production"),
        }
    }
}

impl Stage {
    pub fn try_from(state: &str) -> Result<Self> {
        match state {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid Stage")),
        }
    }
}

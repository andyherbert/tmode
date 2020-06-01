use serde::{Deserialize, Serialize};
pub use std::str::FromStr;
use crate::sauce::error::SauceError;

#[derive(Deserialize, Serialize, Clone)]
pub enum AspectRatio {
    Modern,
    Legacy,
}

impl Default for AspectRatio {
    fn default() -> AspectRatio {
        AspectRatio::Modern
    }
}

impl std::fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AspectRatio::Modern => write!(f, "modern")?,
            AspectRatio::Legacy => write!(f, "legacy")?,
        }
        Ok(())
    }
}

impl FromStr for AspectRatio {
    type Err = Box<SauceError>;
    fn from_str(string: &str) -> Result<AspectRatio, Box<SauceError>> {
        match string {
            "modern" => Ok(AspectRatio::Modern),
            "legacy" => Ok(AspectRatio::Legacy),
            _ => Err(Box::new(SauceError::InvalidAspectRatioValue)),
        }
    }
}

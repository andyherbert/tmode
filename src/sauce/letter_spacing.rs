use crate::sauce::error::SauceError;
use serde::{Deserialize, Serialize};
pub use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone)]
pub enum LetterSpacing {
    EightPixels,
    NinePixels,
}

impl Default for LetterSpacing {
    fn default() -> LetterSpacing {
        LetterSpacing::EightPixels
    }
}

impl std::fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LetterSpacing::EightPixels => write!(f, "8px")?,
            LetterSpacing::NinePixels => write!(f, "9px")?,
        }
        Ok(())
    }
}

impl FromStr for LetterSpacing {
    type Err = Box<SauceError>;
    fn from_str(string: &str) -> Result<LetterSpacing, Box<SauceError>> {
        match string {
            "8px" => Ok(LetterSpacing::EightPixels),
            "9px" => Ok(LetterSpacing::NinePixels),
            _ => Err(Box::new(SauceError::InvalidLetterSpacingValue)),
        }
    }
}

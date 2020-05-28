use serde::{Deserialize, Serialize};
pub use std::str::FromStr;
use super::error::*;

#[derive(Deserialize, Serialize)]
pub enum DataType {
    Character,
    Bitmap,
    Vector,
    Audio,
    BinaryText,
    XBin,
    Archive,
    Executable,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Character => write!(f, "Character"),
            DataType::Bitmap => write!(f, "Bitmap"),
            DataType::Vector => write!(f, "Vector"),
            DataType::Audio => write!(f, "Audio"),
            DataType::BinaryText => write!(f, "Binary Text"),
            DataType::XBin => write!(f, "XBin"),
            DataType::Archive => write!(f, "Archive"),
            DataType::Executable => write!(f, "Executable"),
        }
    }
}

impl DataType {
    pub fn as_u8(&self) -> u8 {
        match self {
            DataType::Character => 1,
            DataType::Bitmap => 2,
            DataType::Vector => 3,
            DataType::Audio => 4,
            DataType::BinaryText => 5,
            DataType::XBin => 6,
            DataType::Archive => 7,
            DataType::Executable => 8,
        }
    }
}

impl FromStr for DataType {
    type Err = Box<SauceError>;
    fn from_str(string: &str) -> Result<DataType, Box<SauceError>> {
        match string {
            "Character" => Ok(DataType::Character),
            "Bitmap" => Ok(DataType::Bitmap),
            "Vector" => Ok(DataType::Vector),
            "Audio" => Ok(DataType::Audio),
            "Binary Text" => Ok(DataType::BinaryText),
            "XBin" => Ok(DataType::XBin),
            "Archive" => Ok(DataType::Archive),
            "Executable" => Ok(DataType::Executable),
            _ => Err(Box::new(SauceError::InvalidDataType)),
        }
    }
}

pub trait AsDataType {
    fn as_datatype(&self) -> Result<Option<DataType>, Box<SauceError>>;
}

impl AsDataType for u8 {
    fn as_datatype(&self) -> Result<Option<DataType>, Box<SauceError>> {
        match self {
            0 => Ok(None),
            1 => Ok(Some(DataType::Character)),
            2 => Ok(Some(DataType::Bitmap)),
            3 => Ok(Some(DataType::Vector)),
            4 => Ok(Some(DataType::Audio)),
            5 => Ok(Some(DataType::BinaryText)),
            6 => Ok(Some(DataType::XBin)),
            7 => Ok(Some(DataType::Archive)),
            8 => Ok(Some(DataType::Executable)),
            _ => Err(Box::new(SauceError::InvalidDataType)),
        }
    }
}

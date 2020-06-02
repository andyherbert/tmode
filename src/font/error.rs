pub use std::error::Error;

#[derive(Debug)]
pub enum FontError {
    FontNotFound,
    ImageNotCorrectDimensions,
    FileNotCorrectSize,
    InvalidXBinFile,
    InvalidADFFile,
    InvalidIDFFile,
    NoFontDataFound,
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FontError::FontNotFound => writeln!(f, "Font not found"),
            FontError::ImageNotCorrectDimensions => writeln!(f, "Image not correct dimensions"),
            FontError::FileNotCorrectSize => writeln!(f, "File not correct size"),
            FontError::InvalidXBinFile => writeln!(f, "Invalid XBin file"),
            FontError::InvalidADFFile => writeln!(f, "Invalid ADF file"),
            FontError::InvalidIDFFile => writeln!(f, "Invalid IDF file"),
            FontError::NoFontDataFound => writeln!(f, "No font data found"),
        }
    }
}

impl Error for FontError {}

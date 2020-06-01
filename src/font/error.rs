pub use std::error::Error;

#[derive(Debug)]
pub enum FontError {
    FontNotFound,
    ImageNotCorrectDimensions,
    FileNotCorrectSize,
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FontError::FontNotFound => writeln!(f, "Font not found"),
            FontError::ImageNotCorrectDimensions => writeln!(f, "Image not correct dimensions"),
            FontError::FileNotCorrectSize => writeln!(f, "File not correct size"),
        }
    }
}

impl Error for FontError {}

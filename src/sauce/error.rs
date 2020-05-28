pub use std::error::Error;

#[derive(Debug)]
pub enum SauceError {
    EOFValueNotFound,
    InvalidDataType,
    InvalidFileType,
    CommentsNotFound,
    InvalidLetterSpacingValue,
    InvalidAspectRatioValue,
    CommentsTooLarge,
}

impl std::fmt::Display for SauceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SauceError::EOFValueNotFound => writeln!(f, "EOF value not found"),
            SauceError::InvalidDataType => writeln!(f, "Invalid datatype"),
            SauceError::InvalidFileType => writeln!(f, "Invalid filetype"),
            SauceError::CommentsNotFound => writeln!(f, "Comments not found"),
            SauceError::InvalidLetterSpacingValue => writeln!(f, "Invalid letter spacing value"),
            SauceError::InvalidAspectRatioValue => writeln!(f, "Invalid aspect ratio value"),
            SauceError::CommentsTooLarge => writeln!(f, "Comments too large"),
        }
    }
}

impl Error for SauceError {}

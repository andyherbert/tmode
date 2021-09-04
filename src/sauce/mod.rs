mod aspect_ratio;
mod datatype;
mod error;
mod filetype;
mod letter_spacing;
pub use self::aspect_ratio::AspectRatio;
pub use self::datatype::{AsDataType, DataType};
use self::error::SauceError;
pub use self::filetype::{AsFileType, FileType};
pub use self::letter_spacing::LetterSpacing;
use crate::ascii;
use crate::bytes::*;
use crate::string::*;
use error::Error;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Sauce {
    pub file: Option<String>,
    pub title: String,
    pub author: String,
    pub group: String,
    pub year: String,
    pub month: String,
    pub date: String,
    filesize: usize,
    actual_filesize: usize,
    pub datatype: Option<DataType>,
    pub filetype: Option<FileType>,
    pub info_1: usize,
    pub info_2: usize,
    pub info_3: usize,
    pub info_4: usize,
    pub ice_colors: bool,
    pub letter_spacing: Option<LetterSpacing>,
    pub aspect_ratio: Option<AspectRatio>,
    pub font_name: String,
    pub comments: Option<String>,
}

impl Sauce {
    pub fn new() -> Sauce {
        Default::default()
    }

    pub fn from_file(file: &str) -> Result<Option<Sauce>, Box<dyn Error>> {
        let bytes = read_file_to_bytes(file)?;
        if let Some(mut sauce) = Sauce::from_bytes(&bytes)? {
            sauce.file = Some(file.to_string());
            Ok(Some(sauce))
        } else {
            Ok(None)
        }
    }

    pub fn remove_from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(sauce) = Sauce::from_bytes(bytes)? {
            Ok(bytes[0..sauce.actual_filesize].to_vec())
        } else {
            Ok(bytes.to_vec())
        }
    }

    pub fn remove_from_file(file: &str) -> Result<(), Box<dyn Error>> {
        let bytes = read_file_to_bytes(file)?;
        let bytes = Sauce::remove_from_bytes(&bytes)?;
        write_bytes_to_file(&bytes, file)?;
        Ok(())
    }

    pub fn add_to_bytes(&mut self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        if let Some(sauce) = Sauce::from_bytes(bytes)? {
            bytes.resize(sauce.actual_filesize, 0);
            self.filesize = sauce.actual_filesize;
        } else {
            self.filesize = bytes.len();
        }
        let mut sauce_bytes = self.to_bytes()?;
        bytes.append(&mut sauce_bytes);
        Ok(())
    }

    pub fn add_to_file(&mut self, file: &str) -> Result<(), Box<dyn Error>> {
        let mut bytes = read_file_to_bytes(file)?;
        self.add_to_bytes(&mut bytes)?;
        write_bytes_to_file(&bytes, file)?;
        Ok(())
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut bytes = vec![ascii::EOF];
        let mut comments_length = 0;
        if let Some(comments) = &self.comments {
            let mut comments_bytes = comments.as_cp437_bytes();
            comments_length = (comments_bytes.len() as f32 / 64.0).ceil() as usize;
            if comments_length > 255 {
                return Err(Box::new(SauceError::CommentsTooLarge));
            }
            comments_bytes.pad_with_spaces(comments_length * 64);
            bytes.resize(134 + comments_bytes.len(), ascii::NULL);
            String::from("COMNT")
                .as_cp437_bytes()
                .write_to_slice(&mut bytes[1..=5])?;
            comments_bytes.write_to_slice(&mut bytes[6..6 + comments_bytes.len()])?;
        } else {
            bytes.resize(129, ascii::NULL);
        }
        let sauce_start = bytes.len() - 128;
        let sauce_bytes = &mut bytes[sauce_start..];
        String::from("SAUCE00")
            .as_cp437_bytes()
            .write_to_slice(&mut sauce_bytes[0..=6])?;
        self.title
            .as_cp437_bytes()
            .pad_with_spaces(35)
            .write_to_slice(&mut sauce_bytes[7..=41])?;
        self.author
            .as_cp437_bytes()
            .pad_with_spaces(20)
            .write_to_slice(&mut sauce_bytes[42..=61])?;
        self.group
            .as_cp437_bytes()
            .pad_with_spaces(20)
            .write_to_slice(&mut sauce_bytes[62..=81])?;
        self.year
            .as_cp437_bytes()
            .write_to_slice(&mut sauce_bytes[82..=85])?;
        self.month
            .as_cp437_bytes()
            .write_to_slice(&mut sauce_bytes[86..=87])?;
        self.date
            .as_cp437_bytes()
            .write_to_slice(&mut sauce_bytes[88..=89])?;
        self.filesize.pack_to_bytes(&mut sauce_bytes[90..=93]);
        sauce_bytes[94] = match &self.datatype {
            Some(datatype) => datatype.as_u8(),
            None => 0,
        };
        if let Some(filetype) = &self.filetype {
            sauce_bytes[95] = filetype.as_u8();
        }
        self.info_1.pack_to_bytes(&mut sauce_bytes[96..=97]);
        self.info_2.pack_to_bytes(&mut sauce_bytes[98..=99]);
        self.info_3.pack_to_bytes(&mut sauce_bytes[100..=101]);
        self.info_4.pack_to_bytes(&mut sauce_bytes[102..=103]);
        sauce_bytes[104] = comments_length as u8;
        if self.ice_colors {
            sauce_bytes[105] = 1;
        }
        if let Some(letter_spacing) = &self.letter_spacing {
            sauce_bytes[105] += match letter_spacing {
                LetterSpacing::EightPixels => 1 << 1,
                LetterSpacing::NinePixels => 2 << 1,
            };
        }
        if let Some(aspect_ratio) = &self.aspect_ratio {
            sauce_bytes[105] += match aspect_ratio {
                AspectRatio::Modern => 2 << 3,
                AspectRatio::Legacy => 1 << 3,
            }
        };
        self.font_name
            .as_cp437_bytes()
            .pad_with_nulls(22)
            .write_to_slice(&mut sauce_bytes[106..=127])?;
        Ok(bytes.to_vec())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Sauce>, Box<dyn Error>> {
        let mut sauce = Sauce::new();
        if bytes.len() < 129 {
            return Ok(None);
        }
        let sauce_start = bytes.len() - 128;
        let sauce_bytes = &bytes[sauce_start..];
        let id = String::from_cp437_bytes(&sauce_bytes[0..=6].to_vec());
        if id != "SAUCE00" {
            return Ok(None);
        }
        sauce.title =
            String::from_cp437_bytes(sauce_bytes[7..=41].to_vec().strip_trailing_spaces());
        sauce.author =
            String::from_cp437_bytes(sauce_bytes[42..=61].to_vec().strip_trailing_spaces());
        sauce.group =
            String::from_cp437_bytes(sauce_bytes[62..=81].to_vec().strip_trailing_spaces());
        sauce.year = String::from_cp437_bytes(&sauce_bytes[82..=85].to_vec());
        sauce.month = String::from_cp437_bytes(&sauce_bytes[86..=87].to_vec());
        sauce.date = String::from_cp437_bytes(&sauce_bytes[88..=89].to_vec());
        sauce.filesize = sauce_bytes[90..=93].as_usize();
        sauce.datatype = sauce_bytes[94].as_datatype()?;
        sauce.filetype = sauce_bytes[95].as_filetype(&sauce.datatype)?;
        sauce.info_1 = sauce_bytes[96..=97].as_usize();
        sauce.info_2 = sauce_bytes[98..=99].as_usize();
        sauce.info_3 = sauce_bytes[100..=101].as_usize();
        sauce.info_4 = sauce_bytes[102..=103].as_usize();
        let lines_of_comments = sauce_bytes[104] as usize;
        let flags = sauce_bytes[105];
        sauce.ice_colors = (flags & 1) == 1;
        sauce.letter_spacing = match (flags >> 1) & 3 {
            0 => None,
            1 => Some(LetterSpacing::EightPixels),
            2 => Some(LetterSpacing::NinePixels),
            _ => return Err(Box::new(SauceError::InvalidLetterSpacingValue)),
        };
        sauce.aspect_ratio = match (flags >> 3) & 3 {
            0 => None,
            1 => Some(AspectRatio::Legacy),
            2 => Some(AspectRatio::Modern),
            _ => return Err(Box::new(SauceError::InvalidAspectRatioValue)),
        };
        sauce.font_name =
            String::from_cp437_bytes(sauce_bytes[106..=127].to_vec().strip_trailing_nulls());
        if lines_of_comments > 0 {
            if bytes.len() < 134 + lines_of_comments * 64 {
                return Err(Box::new(SauceError::CommentsNotFound));
            }
            sauce.actual_filesize = bytes.len() - lines_of_comments * 64 - 134;
            if bytes[sauce.actual_filesize] != ascii::EOF {
                return Err(Box::new(SauceError::EOFValueNotFound));
            }
            let comments_end = bytes.len() - 128;
            let comments_bytes = &bytes[sauce.actual_filesize + 1..comments_end];
            let comment_id = String::from_cp437_bytes(&comments_bytes[0..=4].to_vec());
            if comment_id != "COMNT" {
                return Err(Box::new(SauceError::CommentsNotFound));
            }
            let mut comments = String::new();
            for i in 0..lines_of_comments {
                let comment_start = 5 + i * 64;
                let mut vec = comments_bytes[comment_start..comment_start + 64].to_vec();
                if i == lines_of_comments - 1 {
                    vec.strip_trailing_spaces();
                }
                comments.push_str(&String::from_cp437_bytes(&vec));
            }
            sauce.comments = Some(comments);
        } else {
            sauce.actual_filesize = bytes.len() - 129;
            if bytes[sauce.actual_filesize] != ascii::EOF {
                return Err(Box::new(SauceError::EOFValueNotFound));
            }
        }
        Ok(Some(sauce))
    }
}

impl std::fmt::Display for Sauce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.file {
            Some(file) => writeln!(f, "file: {}", file)?,
            None => writeln!(f, "file: None")?,
        }
        writeln!(f, "title: {}", self.title)?;
        writeln!(f, "author: {}", self.author)?;
        writeln!(f, "group: {}", self.group)?;
        writeln!(f, "year: {}", self.year)?;
        writeln!(f, "month: {}", self.month)?;
        writeln!(f, "date: {}", self.date)?;
        writeln!(f, "filesize: {}", self.filesize)?;
        writeln!(f, "actual filesize: {}", self.actual_filesize)?;
        match &self.datatype {
            Some(datatype) => writeln!(f, "datatype: {}", datatype)?,
            None => writeln!(f, "datatype: None")?,
        }
        match &self.filetype {
            Some(filetype) => writeln!(f, "filetype: {}", filetype)?,
            None => writeln!(f, "filetype: None")?,
        }
        writeln!(f, "type info 1: {}", self.info_1)?;
        writeln!(f, "type info 2: {}", self.info_2)?;
        writeln!(f, "type info 3: {}", self.info_3)?;
        writeln!(f, "type info 4: {}", self.info_4)?;
        writeln!(f, "ice colors: {}", self.ice_colors)?;
        match &self.letter_spacing {
            Some(letter_spacing) => writeln!(f, "letter spacing: {}", letter_spacing)?,
            None => writeln!(f, "letter spacing not set")?,
        }
        match &self.aspect_ratio {
            Some(modern_aspect_ratio) => writeln!(f, "aspect ratio: {}", modern_aspect_ratio)?,
            None => writeln!(f, "aspect ratio not set")?,
        }
        writeln!(f, "font name: {}", &self.font_name)?;
        match &self.comments {
            Some(comments) => writeln!(f, "comments: {}", comments)?,
            None => writeln!(f, "comments not set")?,
        }
        Ok(())
    }
}

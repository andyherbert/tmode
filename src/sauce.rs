use serde::Serialize;
use serde_json::Result;
use super::string::*;
#[derive(Serialize)]
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
            Self::Character => write!(f, "Character"),
            Self::Bitmap => write!(f, "Bitmap"),
            Self::Vector => write!(f, "Vector"),
            Self::Audio => write!(f, "Audio"),
            Self::BinaryText => write!(f, "Binary Text"),
            Self::XBin => write!(f, "XBin"),
            Self::Archive => write!(f, "Archive"),
            Self::Executable => write!(f, "Executable"),
        }
    }
}
#[derive(Serialize)]
pub enum FileType {
    ASCII, ANSI, ANSIMation, RIPscript, PCBoard, Avatar, HTML, Source, TundraDraw,
    GIF, PCX, LBMOrIFF, TGA, FLI, FLC, BMP, GL, DL, WPGBitmap, PNG, JPGOrJPeg, MPG, AVI,
    DXF, DWG, WPGVector, Studio3DS,
    MOD, Renaissance669, STM, S3M, MTM, FAR, ULT, AMF, DMF, OKT, ROL, CMF, MID, SADT, VOC, WAV, SMP8, SMP8S, SMP16, SMP16S, PATCH8, PATCH16, XM, HSC, IT,
    Variable(u8), ExtendedBin,
    ZIP, ARJ, LZH, ARC, TAR, ZOO, RAR, UC2, PAK, SQZ,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ASCII => write!(f, "ASCII"),
            Self::ANSI => write!(f, "ANSI"),
            Self::ANSIMation => write!(f, "ANSIMation"),
            Self::RIPscript => write!(f, "RIPScript"),
            Self::PCBoard => write!(f, "PCBoard"),
            Self::Avatar => write!(f, "Avatar"),
            Self::HTML => write!(f, "HTML"),
            Self::Source => write!(f, "Source"),
            Self::TundraDraw => write!(f, "TundraDraw"),
            Self::GIF => write!(f, "GIF"),
            Self::PCX => write!(f, "PCX"),
            Self::LBMOrIFF => write!(f, "LBM/IFF"),
            Self::TGA => write!(f, "TGA"),
            Self::FLI => write!(f, "FLI"),
            Self::FLC => write!(f, "FLC"),
            Self::BMP => write!(f, "BMP"),
            Self::GL => write!(f, "GL"),
            Self::DL => write!(f, "DL"),
            Self::WPGBitmap => write!(f, "WPG Bitmap"),
            Self::PNG => write!(f, "PNG"),
            Self::JPGOrJPeg => write!(f, "JPG/JPeg"),
            Self::MPG => write!(f, "MPG"),
            Self::AVI => write!(f, "AVI"),
            Self::DXF => write!(f, "DXF"),
            Self::DWG => write!(f, "DWG"),
            Self::WPGVector => write!(f, "WPG Vector"),
            Self::Studio3DS => write!(f, "3DS"),
            Self::MOD => write!(f, "MOD"),
            Self::Renaissance669 => write!(f, "669"),
            Self::STM => write!(f, "STM"),
            Self::S3M => write!(f, "S3M"),
            Self::MTM => write!(f, "MTM"),
            Self::FAR => write!(f, "FAR"),
            Self::ULT => write!(f, "ULT"),
            Self::AMF => write!(f, "AMF"),
            Self::DMF => write!(f, "DMF"),
            Self::OKT => write!(f, "OKT"),
            Self::ROL => write!(f, "ROL"),
            Self::CMF => write!(f, "CMF"),
            Self::MID => write!(f, "MID"),
            Self::SADT => write!(f, "SADT"),
            Self::VOC => write!(f, "VOC"),
            Self::WAV => write!(f, "WAV"),
            Self::SMP8 => write!(f, "SMP8"),
            Self::SMP8S => write!(f, "SMP8S"),
            Self::SMP16 => write!(f, "SMP16"),
            Self::SMP16S => write!(f, "SMP16S"),
            Self::PATCH8 => write!(f, "PATCH8"),
            Self::PATCH16 => write!(f, "PATCH16"),
            Self::XM => write!(f, "XM"),
            Self::HSC => write!(f, "HSC"),
            Self::IT => write!(f, "IT"),
            Self::Variable(value) => write!(f, "Variable({})", value),
            Self::ExtendedBin => write!(f, "ExtendedBin"),
            Self::ZIP => write!(f, "ZIP"),
            Self::ARJ => write!(f, "ARJ"),
            Self::LZH => write!(f, "LZH"),
            Self::ARC => write!(f, "ARC"),
            Self::TAR => write!(f, "TAR"),
            Self::ZOO => write!(f, "ZOO"),
            Self::RAR => write!(f, "RAR"),
            Self::UC2 => write!(f, "UC2"),
            Self::PAK => write!(f, "PAK"),
            Self::SQZ => write!(f, "SQZ"),
        }
    }
}

#[derive(Serialize)]
pub struct Sauce {
    eof: bool,
    id: String,
    comment_id: Option<String>,
    version: String,
    title: String,
    author: String,
    group: String,
    year: Option<usize>,
    month: Option<usize>,
    date: Option<usize>,
    filesize: Option<usize>,
    datatype: Option<DataType>,
    filetype: Option<FileType>,
    type_info_1: Option<usize>,
    type_info_2: Option<usize>,
    type_info_3: Option<usize>,
    type_info_4: Option<usize>,
    lines_of_comments: usize,
    ice_colors: bool,
    letter_spacing: Option<bool>,
    modern_aspect_ratio: Option<bool>,
    font_name: Option<String>,
    comments: Option<String>,
}

impl Sauce {
    fn new() -> Self {
        Self {
            eof: false,
            id: String::new(),
            comment_id: None,
            version: String::new(),
            title: String::new(),
            author: String::new(),
            group: String::new(),
            year: None,
            month: None,
            date: None,
            filesize: None,
            datatype: None,
            filetype: None,
            type_info_1: None,
            type_info_2: None,
            type_info_3: None,
            type_info_4: None,
            lines_of_comments: 0,
            ice_colors: false,
            letter_spacing: None,
            modern_aspect_ratio: None,
            font_name: None,
            comments: None,
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }
}

impl std::fmt::Display for Sauce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "eof: {}", self.eof)?;
        writeln!(f, "sauce id: {}", self.id)?;
        writeln!(f, "version: {}", self.version)?;
        if let Some(comment_id) = &self.comment_id {
            writeln!(f, "comment id: {}", comment_id)?;
        } else {
            writeln!(f, "comment id not set")?;
        }
        writeln!(f, "title: {}", self.title)?;
        writeln!(f, "author: {}", self.author)?;
        writeln!(f, "group: {}", self.group)?;
        if let Some(year) = &self.year {
            writeln!(f, "year: {}", year)?;
        } else {
            writeln!(f, "year not set")?;
        }
        if let Some(month) = &self.month {
            writeln!(f, "month: {}", month)?;
        } else {
            writeln!(f, "month not set")?;
        }
        if let Some(date) = &self.date {
            writeln!(f, "date: {}", date)?;
        } else {
            writeln!(f, "date not set")?;
        }
        if let Some(filesize) = &self.filesize {
            writeln!(f, "filesize: {}", filesize)?;
        } else {
            writeln!(f, "filesize not set")?;
        }
        if let Some(datatype) = &self.datatype {
            writeln!(f, "datatype: {}", datatype)?;
        } else {
            writeln!(f, "datatype not set")?;
        }
        if let Some(filetype) = &self.filetype {
            writeln!(f, "filetype: {}", filetype)?;
        } else {
            writeln!(f, "filetype not set")?;
        }
        if let Some(type_info_1) = self.type_info_1 {
            writeln!(f, "type information 1: {}", type_info_1)?;
        } else {
            writeln!(f, "type information 1 not set")?;
        }
        if let Some(type_info_2) = self.type_info_2 {
            writeln!(f, "type information 2: {}", type_info_2)?;
        } else {
            writeln!(f, "type information 2 not set")?;
        }
        if let Some(type_info_3) = self.type_info_3 {
            writeln!(f, "type information 3: {}", type_info_3)?;
        } else {
            writeln!(f, "type information 3 not set")?;
        }
        if let Some(type_info_4) = self.type_info_4 {
            writeln!(f, "type information 4: {}", type_info_4)?;
        } else {
            writeln!(f, "type information 4 not set")?;
        }
        writeln!(f, "ice colors: {}", self.ice_colors)?;
        if let Some(letter_spacing) = self.letter_spacing {
            writeln!(f, "letter spacing: {}", letter_spacing)?;
        } else {
            writeln!(f, "letter spacing not set")?;
        }
        if let Some(modern_aspect_ratio) = self.modern_aspect_ratio {
            writeln!(f, "modern aspect ratio: {}", modern_aspect_ratio)?;
        } else {
            writeln!(f, "modern aspect ratio not set")?;
        }
        if let Some(font_name) = &self.font_name {
            writeln!(f, "font name: {}", font_name)?;
        } else {
            writeln!(f, "font name not set")?;
        }
        writeln!(f, "comment lines: {}", self.lines_of_comments)?;
        if let Some(comments) = &self.comments {
            writeln!(f, "comments:\n{}", comments)?;
        } else {
            writeln!(f, "comments not set")?;
        }
        Ok(())
    }
}

fn strip_trailing_value(vec: &mut Vec<u8>, value: u8) {
    while let Some(last_value) = vec.last() {
        if *last_value == value {
            vec.pop();
        } else {
            break;
        }
    }
}

fn parse_bytes_to_string_and_strip_trailing_value(bytes: &[u8], value: u8) -> Option<String> {
    let mut vec = bytes.to_vec();
    strip_trailing_value(&mut vec, value);
    match String::from_utf8(vec.clone()) {
        Ok(string) if string.is_empty() => None,
        Ok(string) => Some(string),
        Err(_) => Some(String::from_cp437(vec)),
    }
}

fn parse_bytes_to_usize(bytes: &[u8]) -> Option<usize> {
    let vec = bytes.to_vec();
    match String::from_utf8(vec) {
        Ok(string) if string.is_empty() => None,
        Ok(string) => {
            match string.parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        },
        Err(_) => return None,
    }
}

fn packed_bytes(bytes: &[u8]) -> Option<usize> {
    let mut value: usize = 0;
    for &byte in bytes.iter().rev() {
        value <<= 8;
        value += byte as usize;
    }
    if value > 0 {
        Some(value)
    } else {
        None
    }
}

pub fn parse_bytes(bytes: &Vec<u8>) -> Option<Sauce> {
    let mut sauce = Sauce::new();
    if bytes.len() < 129 {
        return None;
    }
    let sauce_bytes = &bytes[bytes.len() - 128..bytes.len()];
    match parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[0..=4], 0x20) {
        Some(id) if id == "SAUCE" => sauce.id = id,
        _ => return None,
    }
    match parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[5..=6], 0x20) {
        Some(version) if version == "00" => sauce.version = version,
        _ => return None,
    }
    if let Some(title) = parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[7..=41], 0x20) {
        sauce.title = title;
    }
    if let Some(author) = parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[42..=61], 0x20) {
        sauce.author = author;
    }
    if let Some(group) = parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[62..=81], 0x20) {
        sauce.group = group;
    }
    sauce.year = parse_bytes_to_usize(&sauce_bytes[82..=85]);
    sauce.month = parse_bytes_to_usize(&sauce_bytes[86..=87]);
    sauce.date = parse_bytes_to_usize(&sauce_bytes[88..=89]);
    sauce.filesize = packed_bytes(&sauce_bytes[90..=93]);
    sauce.datatype = match sauce_bytes[94] {
        1 => Some(DataType::Character),
        2 => Some(DataType::Bitmap),
        3 => Some(DataType::Vector),
        4 => Some(DataType::Audio),
        5 => Some(DataType::BinaryText),
        6 => Some(DataType::XBin),
        7 => Some(DataType::Archive),
        8 => Some(DataType::Executable),
        0 => None,
        _ => return None,
    };
    sauce.filetype = match sauce.datatype {
        Some(DataType::Character) => {
            match sauce_bytes[95] {
                0 => Some(FileType::ASCII),
                1 => Some(FileType::ANSI),
                2 => Some(FileType::ANSIMation),
                3 => Some(FileType::RIPscript),
                4 => Some(FileType::PCBoard),
                5 => Some(FileType::Avatar),
                6 => Some(FileType::HTML),
                7 => Some(FileType::Source),
                8 => Some(FileType::TundraDraw),
                _ => return None,
            }
        },
        Some(DataType::Bitmap) => {
            match sauce_bytes[95] {
                0 => Some(FileType::GIF),
                1 => Some(FileType::PCX),
                2 => Some(FileType::LBMOrIFF),
                3 => Some(FileType::TGA),
                4 => Some(FileType::FLI),
                5 => Some(FileType::FLC),
                6 => Some(FileType::BMP),
                7 => Some(FileType::GL),
                8 => Some(FileType::DL),
                9 => Some(FileType::WPGBitmap),
                10 => Some(FileType::PNG),
                11 => Some(FileType::JPGOrJPeg),
                12 => Some(FileType::MPG),
                13 => Some(FileType::AVI),
                16 => Some(FileType::GIF),
                _ => return None,
            }
        },
        Some(DataType::Vector) => {
            match sauce_bytes[95] {
                0 => Some(FileType::DXF),
                1 => Some(FileType::DWG),
                2 => Some(FileType::WPGVector),
                3 => Some(FileType::Studio3DS),
                _ => return None,
            }
        },
        Some(DataType::Audio) => {
            match sauce_bytes[95] {
                0 => Some(FileType::MOD),
                1 => Some(FileType::Renaissance669),
                2 => Some(FileType::STM),
                3 => Some(FileType::S3M),
                4 => Some(FileType::MTM),
                5 => Some(FileType::FAR),
                6 => Some(FileType::ULT),
                7 => Some(FileType::AMF),
                8 => Some(FileType::DMF),
                9 => Some(FileType::OKT),
                10 => Some(FileType::ROL),
                11 => Some(FileType::CMF),
                12 => Some(FileType::MID),
                13 => Some(FileType::SADT),
                14 => Some(FileType::VOC),
                15 => Some(FileType::WAV),
                16 => Some(FileType::SMP8),
                17 => Some(FileType::SMP8S),
                18 => Some(FileType::SMP16),
                19 => Some(FileType::SMP16S),
                20 => Some(FileType::PATCH8),
                21 => Some(FileType::PATCH16),
                22 => Some(FileType::XM),
                23 => Some(FileType::HSC),
                24 => Some(FileType::IT),
                _ => return None,
            }
        },
        Some(DataType::BinaryText) => Some(FileType::Variable(bytes[95])),
        Some(DataType::XBin) => {
            if bytes[95] != 0 {
                return None;
            }
            Some(FileType::ExtendedBin)
        },
        Some(DataType::Archive) => {
            match sauce_bytes[95] {
                0 => Some(FileType::ZIP),
                1 => Some(FileType::ARJ),
                2 => Some(FileType::LZH),
                3 => Some(FileType::ARC),
                4 => Some(FileType::TAR),
                5 => Some(FileType::ZOO),
                6 => Some(FileType::RAR),
                7 => Some(FileType::UC2),
                8 => Some(FileType::PAK),
                9 => Some(FileType::SQZ),
                _ => return None,
            }
        },
        Some(DataType::Executable) => {
            if bytes[95] != 0 {
                return None;
            }
            None
        },
        None => None,
    };
    sauce.type_info_1 = packed_bytes(&sauce_bytes[96..=97]);
    sauce.type_info_2 = packed_bytes(&sauce_bytes[98..=99]);
    sauce.type_info_3 = packed_bytes(&sauce_bytes[100..=101]);
    sauce.type_info_4 = packed_bytes(&sauce_bytes[102..=103]);
    sauce.lines_of_comments = sauce_bytes[104] as usize;
    let flags = sauce_bytes[105];
    sauce.ice_colors = (flags & 1) == 1;
    sauce.letter_spacing = match (flags >> 1) & 3 {
        0 => None,
        1 => Some(false),
        2 => Some(true),
        _ => return None,
    };
    sauce.modern_aspect_ratio = match (flags >> 5) & 3 {
        0 => None,
        1 => Some(false),
        2 => Some(true),
        _ => return None,
    };
    sauce.font_name = parse_bytes_to_string_and_strip_trailing_value(&sauce_bytes[106..=127], 0x0);
    if sauce.lines_of_comments > 0 {
        if bytes.len() < 134 + sauce.lines_of_comments * 64 {
            return None;
        }
        sauce.eof = bytes[bytes.len() - 134 - sauce.lines_of_comments * 64] == 0x1a;
        let comments_bytes = &bytes[bytes.len() - sauce.lines_of_comments * 64 - 133..bytes.len() - 128];
        match parse_bytes_to_string_and_strip_trailing_value(&comments_bytes[0..=4], 0x20) {
            Some(comment_id) if comment_id == "COMNT" => sauce.comment_id = Some(comment_id),
            _ => return None,
        }
        let mut comments = String::new();
        for i in 0..sauce.lines_of_comments {
            let byte_index = 5 + i * 64;
            let mut vec = comments_bytes[byte_index..byte_index + 64].to_vec();
            if i == sauce.lines_of_comments - 1 {
                strip_trailing_value(&mut vec, 0x20);
            }
            match String::from_utf8(vec.clone()) {
                Ok(string) => comments.push_str(&string),
                Err(_) => comments.push_str(&String::from_cp437(vec)),
            }
        }
        sauce.comments = Some(comments);
    } else {
        sauce.eof = bytes[bytes.len() - 129] == 0x1a;
    }
    Some(sauce)
}

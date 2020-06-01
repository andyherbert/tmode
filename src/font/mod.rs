use crate::render::{Image, Color};
use std::error::Error;
pub use std::str::FromStr;
mod error;
mod includes;
use includes::*;
use error::FontError;
use crate::bytes::{read_file_to_bytes, write_bytes_to_file};


pub struct Font {
    pub width: usize,
    pub height: usize,
    pub length: usize,
    pub bitmasks: Vec<Vec<bool>>,
}

pub fn get_boolmasks(bitmask: &[u8], width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut boolmask = Vec::new();
    for byte in bitmask {
        for i in (0..8).rev() {
            let bool = ((byte >> i) & 1) == 1;
            boolmask.push(bool);
        }
    }
    let glyph_len = width * height;
    let mut boolmasks = Vec::new();
    for glyph_start in (0..boolmask.len()).step_by(glyph_len) {
        let mut glyph_mask = Vec::new();
        for i in glyph_start..glyph_start + glyph_len {
            glyph_mask.push(boolmask[i]);
        }
        boolmasks.push(glyph_mask);
    }
    boolmasks
}

impl Font {
    pub fn new(bitmask: &[u8], width: usize, height: usize, length: usize) -> Font {
        Font {
            width,
            height,
            length,
            bitmasks: get_boolmasks(bitmask, width, height),
        }
    }

    pub fn from_bitmask_file(file: &str, width: usize, height: Option<usize>, length: usize) -> Result<Font, Box<dyn Error>> {
        let bytes = read_file_to_bytes(file)?;
        let height = match height {
            Some(height) => height,
            None => bytes.len() * 8 / length / width,
        };
        if bytes.len() * 8 < width * height * length {
            return Err(Box::new(FontError::FileNotCorrectSize));
        }
        let font = Font::new(&bytes, width, height, length);
        Ok(font)
    }

    pub fn from_png_file(file: &str, width: usize, height: Option<usize>, length: usize) -> Result<Font, Box<dyn Error>> {
        let image = Image::from_file(file)?;
        let height = match height {
            Some(height) => height,
            None => image.height / (image.width / width),
        };
        if image.width % width != 0 || image.height % height != 0 || (image.width / width) * (image.height / height) < length {
            return Err(Box::new(FontError::ImageNotCorrectDimensions));
        }
        let mut glyph_x = 0;
        let mut glyph_y = 0;
        let mut boolmask = Vec::with_capacity(image.width * image.height);
        for _ in 0..length {
            for y in glyph_y..glyph_y + height {
                for x in glyph_x..glyph_x + width {
                    let i = (y * image.width + x) * 3;
                    let avg = (image.data[i] as usize + image.data[i + 1]  as usize + image.data[i + 2]  as usize) / 3;
                    boolmask.push(avg > 127);
                }
            }
            if glyph_x + width == image.width {
                glyph_y += height;
                glyph_x = 0;
            } else {
                glyph_x += width;
            }
        }
        let mut bitmask = Vec::with_capacity(boolmask.len() / 8);
        for i in (0..boolmask.len()).step_by(8) {
            let mut byte: u8 = 0;
            for j in i..i + 8 {
                byte <<= 1;
                byte += if boolmask[j] {
                    1
                } else {
                    0
                }
            }
            bitmask.push(byte);
        }
        let font = Font::new(&bitmask, width, height, length);
        Ok(font)
    }

    pub fn as_png_file(&self, file: &str, width: usize) -> Result<(), Box<dyn Error>> {
        let mut image = Image::new(width * self.width, (self.length as f32 / width as f32).ceil() as usize * self.height);
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        let mut x = 0;
        let mut y = 0;
        for code in 0..self.length {
            image.draw_font(x * self.width, y * self.height, &self, code, &white.rgb, &black.rgb);
            if x + 1 == width {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }
        image.as_png(file)?;
        Ok(())
    }

    pub fn as_bitmask_file(&self, file: &str) -> Result<(), Box<dyn Error>> {
        let bytes_len = (self.width as f32 * self.height as f32 * self.length as f32 / 8.0).ceil() as usize;
        let bits_len = bytes_len * 8;
        let mut bitmasks = self.bitmasks.concat();
        let mut bytes = Vec::with_capacity(bytes_len);
        if bitmasks.len() != bits_len * 8 {
            bitmasks.resize(bits_len, false)
        }
        for byte_start in (0..bitmasks.len()).step_by(8) {
            let mut byte: u8 = 0;
            for i in byte_start..byte_start + 8 {
                byte <<= 1;
                byte += if bitmasks[i] {
                    1
                } else {
                    0
                }
            }
            bytes.push(byte);
        }
        write_bytes_to_file(&bytes, file)?;
        Ok(())
    }
}

impl FromStr for Font {
    type Err = Box<dyn Error>;
    fn from_str(string: &str) -> Result<Font, Box<dyn Error>> {
        match string {
            "IBM VGA" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 437" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 437" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 437" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 437" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 437" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 720" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 720" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 720" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 720" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 720" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 737" => Ok(Font::new(CP737_F16, 8, 16, 256)),
            "IBM VGA50 737" => Ok(Font::new(CP737_F08, 8, 8, 256)),
            "IBM VGA25G 737" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 737" => Ok(Font::new(CP737_F14, 8, 14, 256)),
            "IBM EGA43 737" => Ok(Font::new(CP737_F08, 8, 8, 256)),
            "IBM VGA 775" => Ok(Font::new(CP775_F16, 8, 16, 256)),
            "IBM VGA50 775" => Ok(Font::new(CP775_F08, 8, 8, 256)),
            "IBM VGA25G 775" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 775" => Ok(Font::new(CP775_F14, 8, 14, 256)),
            "IBM EGA43 775" => Ok(Font::new(CP775_F08, 8, 8, 256)),
            "IBM VGA 819" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 819" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 819" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 819" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 819" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 850" => Ok(Font::new(CP850_F16, 8, 16, 256)),
            "IBM VGA50 850" => Ok(Font::new(CP850_F08, 8, 8, 256)),
            "IBM VGA25G 850" => Ok(Font::new(CP850_F19, 8, 19, 256)),
            "IBM EGA 850" => Ok(Font::new(CP850_F14, 8, 14, 256)),
            "IBM EGA43 850" => Ok(Font::new(CP850_F08, 8, 8, 256)),
            "IBM VGA 852" => Ok(Font::new(CP852_F16, 8, 16, 256)),
            "IBM VGA50 852" => Ok(Font::new(CP852_F08, 8, 8, 256)),
            "IBM VGA25G 852" => Ok(Font::new(CP852_F19, 8, 19, 256)),
            "IBM EGA 852" => Ok(Font::new(CP852_F14, 8, 14, 256)),
            "IBM EGA43 852" => Ok(Font::new(CP852_F08, 8, 8, 256)),
            "IBM VGA 855" => Ok(Font::new(CP855_F16, 8, 16, 256)),
            "IBM VGA50 855" => Ok(Font::new(CP855_F08, 8, 8, 256)),
            "IBM VGA25G 855" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 855" => Ok(Font::new(CP855_F14, 8, 14, 256)),
            "IBM EGA43 855" => Ok(Font::new(CP855_F08, 8, 8, 256)),
            "IBM VGA 857" => Ok(Font::new(CP857_F16, 8, 16, 256)),
            "IBM VGA50 857" => Ok(Font::new(CP857_F08, 8, 8, 256)),
            "IBM VGA25G 857" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 857" => Ok(Font::new(CP857_F14, 8, 14, 256)),
            "IBM EGA43 857" => Ok(Font::new(CP857_F08, 8, 8, 256)),
            "IBM VGA 858" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 858" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 858" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 858" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 858" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 860" => Ok(Font::new(CP860_F16, 8, 16, 256)),
            "IBM VGA50 860" => Ok(Font::new(CP860_F08, 8, 8, 256)),
            "IBM VGA25G 860" => Ok(Font::new(CP860_F19, 8, 19, 256)),
            "IBM EGA 860" => Ok(Font::new(CP860_F14, 8, 14, 256)),
            "IBM EGA43 860" => Ok(Font::new(CP860_F08, 8, 8, 256)),
            "IBM VGA 861" => Ok(Font::new(CP861_F16, 8, 16, 256)),
            "IBM VGA50 861" => Ok(Font::new(CP861_F08, 8, 8, 256)),
            "IBM VGA25G 861" => Ok(Font::new(CP861_F19, 8, 19, 256)),
            "IBM EGA 861" => Ok(Font::new(CP861_F14, 8, 14, 256)),
            "IBM EGA43 861" => Ok(Font::new(CP861_F08, 8, 8, 256)),
            "IBM VGA 862" => Ok(Font::new(CP862_F16, 8, 16, 256)),
            "IBM VGA50 862" => Ok(Font::new(CP862_F08, 8, 8, 256)),
            "IBM VGA25G 862" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 862" => Ok(Font::new(CP862_F14, 8, 14, 256)),
            "IBM EGA43 862" => Ok(Font::new(CP862_F08, 8, 8, 256)),
            "IBM VGA 863" => Ok(Font::new(CP863_F16, 8, 16, 256)),
            "IBM VGA50 863" => Ok(Font::new(CP863_F08, 8, 8, 256)),
            "IBM VGA25G 863" => Ok(Font::new(CP863_F19, 8, 19, 256)),
            "IBM EGA 863" => Ok(Font::new(CP863_F14, 8, 14, 256)),
            "IBM EGA43 863" => Ok(Font::new(CP863_F08, 8, 8, 256)),
            "IBM VGA 864" => Ok(Font::new(CP864_F16, 8, 16, 256)),
            "IBM VGA50 864" => Ok(Font::new(CP864_F08, 8, 8, 256)),
            "IBM VGA25G 864" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 864" => Ok(Font::new(CP864_F14, 8, 14, 256)),
            "IBM EGA43 864" => Ok(Font::new(CP864_F08, 8, 8, 256)),
            "IBM VGA 865" => Ok(Font::new(CP865_F16, 8, 16, 256)),
            "IBM VGA50 865" => Ok(Font::new(CP865_F08, 8, 8, 256)),
            "IBM VGA25G 865" => Ok(Font::new(CP865_F19, 8, 19, 256)),
            "IBM EGA 865" => Ok(Font::new(CP865_F14, 8, 14, 256)),
            "IBM EGA43 865" => Ok(Font::new(CP865_F08, 8, 8, 256)),
            "IBM VGA 869" => Ok(Font::new(CP869_F16, 8, 16, 256)),
            "IBM VGA50 869" => Ok(Font::new(CP869_F08, 8, 8, 256)),
            "IBM VGA25G 869" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 869" => Ok(Font::new(CP869_F14, 8, 14, 256)),
            "IBM EGA43 869" => Ok(Font::new(CP869_F08, 8, 8, 256)),
            "IBM VGA 872" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 872" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 872" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 872" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 872" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA KAM" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 KAM" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G KAM" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA KAM" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 KAM" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA MAZ" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 MAZ" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G MAZ" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA MAZ" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 MAZ" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA MIK" => Ok(Font::new(CP866_F16, 8, 16, 256)),
            "IBM VGA50 MIK" => Ok(Font::new(CP866_F08, 8, 8, 256)),
            "IBM VGA25G MIK" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA MIK" => Ok(Font::new(CP866_F14, 8, 14, 256)),
            "IBM EGA43 MIK" => Ok(Font::new(CP866_F08, 8, 8, 256)),
            "IBM VGA 667" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 667" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 667" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 667" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 667" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 790" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 790" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 790" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 790" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 790" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 866" => Ok(Font::new(CP866_F16, 8, 16, 256)),
            "IBM VGA50 866" => Ok(Font::new(CP866_F08, 8, 8, 256)),
            "IBM VGA25G 866" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 866" => Ok(Font::new(CP866_F14, 8, 14, 256)),
            "IBM EGA43 866" => Ok(Font::new(CP866_F08, 8, 8, 256)),
            "IBM VGA 867" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 867" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 867" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 867" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 867" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 895" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 895" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 895" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 895" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 895" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA 991" => Ok(Font::new(CP437_F16, 8, 16, 256)),
            "IBM VGA50 991" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "IBM VGA25G 991" => Ok(Font::new(CP437_F19, 8, 19, 256)),
            "IBM EGA 991" => Ok(Font::new(CP437_F14, 8, 14, 256)),
            "IBM EGA43 991" => Ok(Font::new(CP437_F08, 8, 8, 256)),
            "Amiga Topaz 1" => Ok(Font::new(TOPAZ_A500_F16, 8, 16, 256)),
            "Amiga Topaz 1+" => Ok(Font::new(TOPAZ_PLUS_A500_F16, 8, 16, 256)),
            "Amiga Topaz 2" => Ok(Font::new(TOPAZ_A1200_F16, 8, 16, 256)),
            "Amiga Topaz 2+" => Ok(Font::new(TOPAZ_PLUS_A1200_F16, 8, 16, 256)),
            "Amiga P0T-NOoDLE" => Ok(Font::new(P0T_NOODLE_F16, 8, 16, 256)),
            "Amiga MicroKnight" => Ok(Font::new(MICRO_KNIGHT_F16, 8, 16, 256)),
            "Amiga MicroKnight+" => Ok(Font::new(MICRO_KNIGHT_PLUS_F16, 8, 16, 256)),
            "Amiga mOsOul" => Ok(Font::new(MO_SOUL_F16, 8, 16, 256)),
            "C64 PETSCII unshifted" => Ok(Font::new(PETSCII_UNSHIFTED, 8, 8, 256)),
            "C64 PETSCII shifted" => Ok(Font::new(PETSCII_SHIFTED, 8, 8, 256)),
            _ => Err(Box::new(FontError::FontNotFound)),
        }
    }
}

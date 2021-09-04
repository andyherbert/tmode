use crate::font::Font;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    line_len: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            data: vec![0; width * height * 3],
            line_len: width * 3,
        }
    }

    pub fn from_file(file: &str) -> Result<Image, Box<dyn Error>> {
        let file = File::open(file)?;
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info()?;
        let mut data = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut data)?;
        let info = reader.info();
        let image = Image {
            width: info.width as usize,
            height: info.height as usize,
            data,
            line_len: (info.width as usize) * 3,
        };
        Ok(image)
    }

    pub fn draw_font(
        &mut self,
        x: usize,
        y: usize,
        font: &Font,
        code: usize,
        fg: &[u8; 3],
        bg: &[u8; 3],
    ) {
        let mut boolmask_index = 0;
        let mut i = (y * self.width + x) * 3;
        let line_len = self.line_len - font.width * 3;
        for _ in 0..font.height {
            for _ in 0..font.width {
                if font.bitmasks[code][boolmask_index] {
                    self.data[i] = fg[0];
                    self.data[i + 1] = fg[1];
                    self.data[i + 2] = fg[2];
                } else {
                    self.data[i] = bg[0];
                    self.data[i + 1] = bg[1];
                    self.data[i + 2] = bg[2];
                }
                boolmask_index += 1;
                i += 3;
            }
            i += line_len;
        }
    }

    pub fn as_png(&self, file: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file)?;
        let buffer = BufWriter::new(file);
        let mut encoder = png::Encoder::new(buffer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Best);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        Ok(())
    }
}

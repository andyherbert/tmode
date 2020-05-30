use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

static CP437_F16: &[u8; 4096] = include_bytes!("../../fonts/ibm/CP437.F16");

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

pub fn test() {
    let path = Path::new("temp.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 8, 16);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut image_data: [u8; 8 * 16 * 3] = [0; 8 * 16 * 3];
    let boolmasks = get_boolmasks(CP437_F16, 8, 16);
    let mut boolmask_index = 0;
    let mut image_data_index = 0;
    for _ in 0..16 {
        for _ in 0..8 {
            if boolmasks[63][boolmask_index] {
                image_data[image_data_index + 0] = 255;
                image_data[image_data_index + 1] = 255;
                image_data[image_data_index + 2] = 255;
            } else {
                image_data[image_data_index + 0] = 0;
                image_data[image_data_index + 1] = 0;
                image_data[image_data_index + 2] = 0;
            }
            boolmask_index += 1;
            image_data_index += 3;
        }
    }
    writer.write_image_data(&image_data).unwrap();
}

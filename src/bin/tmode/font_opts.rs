use clap::{ArgMatches};
use std::error::Error;
use textmode::font::Font;
use std::path::Path;

pub fn font_as_png(font_file: &str, output_file: &str, width: &str, height: Option<&str>, length: &str, chars_per_row: &str) -> Result<(), Box<dyn Error>> {
    let chars_per_row = chars_per_row.parse::<usize>()?;
    let width = width.parse::<usize>()?;
    let height = match height {
        Some(height) => Some(height.parse::<usize>()?),
        None => None,
    };
    let length = length.parse::<usize>()?;
    let font = Font::from_bitmask_file(font_file, width, height, length)?;
    font.as_png_file(output_file, chars_per_row)?;
    Ok(())
}

pub fn png_as_font(png_file: &str, output_file: &str, width: &str, height: Option<&str>, length: &str) -> Result<(), Box<dyn Error>> {
    let width = width.parse::<usize>()?;
    let height = match height {
        Some(height) => Some(height.parse::<usize>()?),
        None => None,
    };
    let length = length.parse::<usize>()?;
    let font = Font::from_png_file(png_file, width, height, length)?;
    font.as_bitmask_file(output_file)?;
    Ok(())
}

pub fn get_font_from_file(file: &str) -> Result<Font, Box<dyn Error>> {
    let path = Path::new(file);
    match path.extension() {
        Some(extension) => {
            match extension.to_str() {
                Some(extension) => {
                    match extension.to_ascii_uppercase().as_str() {
                        "XB" => Font::from_xbin_file(file),
                        "ADF" => Font::from_adf_file(file),
                        "IDF" => Font::from_idf_file(file),
                        _ => Font::from_sauce(file),
                    }
                },
                None => Font::from_sauce(file),
            }
        }
        None => Font::from_sauce(file),
    }
}

pub fn export_font(file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let font = get_font_from_file(file)?;
    font.as_bitmask_file(output_file)?;
    Ok(())
}

pub fn export_font_as_png(file: &str, output_file: &str, chars_per_row: &str) -> Result<(), Box<dyn Error>> {
    let chars_per_row = chars_per_row.parse::<usize>()?;
    let font = get_font_from_file(file)?;
    font.as_png_file(output_file, chars_per_row)?;
    Ok(())
}

pub fn font_opts(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("font_as_png") {
        font_as_png(matches.value_of("files").unwrap(), matches.value_of("font_as_png").unwrap(),  matches.value_of("font_width").unwrap(), matches.value_of("font_height"), matches.value_of("font_length").unwrap(), matches.value_of("chars_per_row").unwrap())?;
    }
    if matches.is_present("png_as_font") {
        png_as_font(matches.value_of("files").unwrap(), matches.value_of("png_as_font").unwrap(), matches.value_of("font_width").unwrap(), matches.value_of("font_height"), matches.value_of("font_length").unwrap())?;
    }
    if matches.is_present("export_font") {
        export_font(matches.value_of("files").unwrap(), matches.value_of("export_font").unwrap())?;
    }
    if matches.is_present("export_font_as_png") {
        export_font_as_png(matches.value_of("files").unwrap(), matches.value_of("export_font_as_png").unwrap(), matches.value_of("chars_per_row").unwrap())?;
    }
    Ok(())
}

use clap::{ArgMatches};
use std::error::Error;
use textmode::font::Font;

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

pub fn font_opts(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("font_as_png") {
        font_as_png(matches.value_of("files").unwrap(), matches.value_of("font_as_png").unwrap(),  matches.value_of("font_width").unwrap(), matches.value_of("font_height"), matches.value_of("font_length").unwrap(), matches.value_of("chars_per_row").unwrap())?;
    }
    if matches.is_present("png_as_font") {
        png_as_font(matches.value_of("files").unwrap(), matches.value_of("png_as_font").unwrap(), matches.value_of("font_width").unwrap(), matches.value_of("font_height"), matches.value_of("font_length").unwrap())?;
    }
    Ok(())
}

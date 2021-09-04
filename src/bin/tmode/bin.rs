mod font_opts;
mod sauce_opts;
use clap::{clap_app, crate_authors, crate_description, crate_version};
use font_opts::font_opts;
use sauce_opts::sauce_opts;
use std::error::Error;
use std::process::exit;
use std::u64;
use wild::args;

fn main() -> Result<(), Box<dyn Error>> {
    let app = clap_app!(tmode =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg current_date: --("current-date") requires("files") "Adds the current local date to SAUCE records.")
        (@arg filetype: --filetype requires("files") "Automatically insert a SAUCE record for non-textmode files.")
        (@arg ice_colors: --("ice-colors") requires("files") "Enables iCE colors for supported filetypes.")
        (@arg legacy_aspect: --("legacy-aspect") requires("files") "Enables legacy aspect ratio for supported filetypes.")
        (@arg letter_spacing: --("letter-spacing") requires("files") "Enables 9px fonts for supported filetypes.")
        (@arg modern_aspect: --("modern-aspect") requires("files") "Enables modern aspect ratio for supported filetypes.")
        (@arg no_ice_colors: --("no-ice-colors") requires("files") "Disables iCE colors for supported filetypes.")
        (@arg no_letter_spacing: --("no-letter-spacing") requires("files") "Disables 9px fonts for supported filetypes.")
        (@arg no_sauce: --("no-sauce") requires("files") "Lists all the files with no SAUCE record.")
        (@arg sauce_display: -s --sauce requires("files") "Displays SAUCE information.")
        (@arg sauce_remove: -r --remove requires("files") "Removes SAUCE records.")
        (@arg unset_aspect: --("unset-aspect") requires("files") "Sets no preference on aspect ratio setting for supported filetypes.")
        (@arg unset_letter_spacing: --("unset-letter-spacing") requires("files") "Sets no preference on letter spacing for supported filetypes.")
        (@arg author: --author +takes_value +require_equals +empty_values value_name("author's name") requires("files") "Adds an author to SAUCE records.")
        (@arg comments: --comments +takes_value +require_equals +empty_values requires("files") "Adds comments to SAUCE records.")
        (@arg day: --day +takes_value +require_equals value_name("day of the month") requires("files") "Adds a day of the month to SAUCE records.")
        (@arg export_csv: --("export-csv") +takes_value +require_equals value_name("CSV file") requires("files") "Exports multiple SAUCE records to a CSV file.")
        (@arg export_font: --("export-font") +takes_value +require_equals value_name("font file") requires("files") "Extracts font data to a bitmask font file.")
        (@arg export_font_as_png: --("export-font-as-png") +takes_value +require_equals value_name("PNG file") requires("files") "Extracts font data to a PNG file.")
        (@arg export_json: --("export-json") +takes_value +require_equals value_name("JSON file") requires("files") "Exports multiple SAUCE records to a JSON file.")
        (@arg font: --font +takes_value +require_equals +empty_values value_name("font name") requires("files") "Adds a font name to SAUCE records.")
        (@arg font_as_png: --("font-as-png") +takes_value +require_equals value_name("PNG file") requires("files") "Converts a bitmask font file to a PNG file.\nUse with --chars-per-row. [default: 16]")
        (@arg font_height: --("font-height") +takes_value +require_equals "Specifies the font height when importing.")
        (@arg font_length: --("font-length") +takes_value +require_equals default_value("256") "Specifies the font length when importing.")
        (@arg font_width: --("font-width") +takes_value +require_equals default_value("8") "Specifies the font width when importing.")
        (@arg group: --group +takes_value +require_equals +empty_values value_name("group's name") requires("files") "Adds a group to SAUCE records.")
        (@arg import_csv: --("import-csv") +takes_value +require_equals value_name("CSV file") "Imports a CSV file to update multiple SAUCE records.")
        (@arg import_json: --("import-json") +takes_value +require_equals value_name("JSON file") "Imports a JSON file to update multiple SAUCE records.")
        (@arg info_1: --("info-1") +takes_value +require_equals value_name("numeric value") requires("files") "Sets a numeric value to information 1.")
        (@arg info_2: --("info-2") +takes_value +require_equals value_name("numeric value") requires("files") "Sets a numeric value to information 2.")
        (@arg info_3: --("info-3") +takes_value +require_equals value_name("numeric value") requires("files") "Sets a numeric value to information 3.")
        (@arg info_4: --("info-4") +takes_value +require_equals value_name("numeric value") requires("files") "Sets a numeric value to information 4.")
        (@arg month: --month +takes_value +require_equals value_name("month of the year") requires("files") "Adds a (numeric) month of the year to SAUCE records.")
        (@arg png_as_font: --("png-as-font") +takes_value +require_equals value_name("font file") requires("files") "Converts a PNG file to a bitmask font file.\nUse with --font-width, --font-height, and --font-length.")
        (@arg title: --title +takes_value +require_equals +empty_values requires("files") "Adds a title to the SAUCE records.")
        (@arg year: --year +takes_value +require_equals requires("file") "Adds a year to SAUCE records.")
        (@arg chars_per_row: --("chars-per-row") +takes_value +require_equals default_value("16") +hidden)
        (@arg files: ... #{1, u64::MAX} value_name("FILE") "Sets the input file(s) to use.")
    );
    let matches = if cfg!(windows) {
        app.get_matches_from(args())
    } else {
        app.get_matches()
    };
    if let Err(e) = sauce_opts(&matches) {
        eprintln!("Error: {}", e);
        exit(1);
    }
    if let Err(e) = font_opts(&matches) {
        eprintln!("Error: {}", e);
        exit(1);
    }
    Ok(())
}

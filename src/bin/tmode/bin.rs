mod sauce_opts;
use std::error::Error;
use clap::{clap_app, crate_version, crate_description, crate_authors};
use std::u64;
use wild::args;
use sauce_opts::*;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let app = clap_app!(tmode =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg current_date: --("current-date") requires("FILE") "Adds the current local date to SAUCE records.")
        (@arg filetype: --filetype requires("FILE") "Automatically insert a SAUCE record for non-textmode files.")
        (@arg no_sauce: --("no-sauce") requires("FILE") "Lists all the files with no SAUCE record.")
        (@arg sauce_display: -s --sauce requires("FILE") "Displays SAUCE information.")
        (@arg sauce_remove: -r --remove requires("FILE") "Removes SAUCE records.")
        (@arg author: --author +takes_value +require_equals +empty_values value_name("author's name") requires("FILE") "Adds an author to SAUCE records.")
        (@arg comments: --comments +takes_value +require_equals +empty_values requires("FILE") "Adds comments to SAUCE records.")
        (@arg day: --day +takes_value +require_equals value_name("day of the month") requires("FILE") "Adds a day of the month to SAUCE records.")
        (@arg export_csv: --("export-csv") +takes_value +require_equals value_name("CSV FILE") requires("FILE") "Exports multiple SAUCE records to a CSV file.")
        (@arg export_json: --("export-json") +takes_value +require_equals value_name("JSON FILE") requires("FILE") "Exports multiple SAUCE records to a JSON file.")
        (@arg font: --font +takes_value +require_equals +empty_values value_name("font name") requires("FILE") "Adds a font name to SAUCE records.")
        (@arg group: --group +takes_value +require_equals +empty_values value_name("group's name") requires("FILE") "Adds a group to SAUCE records.")
        (@arg import_csv: --("import-csv") +takes_value +require_equals value_name("CSV FILE") "Imports a CSV file to update multiple SAUCE records.")
        (@arg import_json: --("import-json") +takes_value +require_equals value_name("JSON FILE") "Imports a JSON file to update multiple SAUCE records.")
        (@arg month: --month +takes_value +require_equals value_name("month of the year") requires("FILE") "Adds a month of the year to SAUCE records.")
        (@arg title: --title +takes_value +require_equals +empty_values requires("FILE") "Adds a title to the SAUCE records.")
        (@arg year: --year +takes_value +require_equals requires("FILE") "Adds a year to SAUCE records.")
        (@arg FILE: ... #{1, u64::MAX} "Sets the input file(s) to use.")
    );
    let matches = if cfg!(windows) {
        app.get_matches_from(args())
    } else {
        app.get_matches()
    };
    if let Err(e) = sauce_options(&matches) {
        eprintln!("Error: {}", e);
        exit(1);
    }
    Ok(())
}

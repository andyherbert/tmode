use clap::{App, Arg, Values};
use clap::{crate_name, crate_version, crate_description, crate_authors};
use textmode::*;
use std::error::Error;

fn display_sauce(values: Option<Values>) {
    match values {
        Some(files) => {
            for file in files {
                match Sauce::from_file(&file) {
                    Ok(sauce) => println!("{}", sauce),
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        },
        None => {
            match Sauce::from_stdin() {
                Ok(sauce) => println!("{}", sauce),
                Err(e) => eprintln!("{}", e),
            }
        },
    }
}

fn display_sauce_as_json(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    match values {
        Some(files) => {
            for file in files {
                match Sauce::from_file(&file) {
                    Ok(sauce) => println!("{}", sauce.to_json()?),
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        },
        None => {
            match Sauce::from_stdin() {
                Ok(sauce) => println!("{}", sauce.to_json()?),
                Err(e) => eprintln!("{}", e),
            }
        },
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT")
            .multiple(true)
            .min_values(1)
            .help("Sets the input file to use, standard input is read if absent."))
        .arg(Arg::with_name("sauce")
            .short("s")
            .long("sauce")
            .help("Displays sauce information."))
        .arg(Arg::with_name("sauce_json")
            .short("j")
            .long("json-sauce")
            .help("Displays sauce information in JSON format."));
    let matches = app.get_matches();
    if matches.is_present("sauce") {
        display_sauce(matches.values_of("INPUT"));
    }
    if matches.is_present("sauce_json") {
        display_sauce_as_json(matches.values_of("INPUT"))?;
    }
    Ok(())
}

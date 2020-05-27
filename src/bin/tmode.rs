use clap::{App, Arg, Values};
use clap::{crate_name, crate_version, crate_description, crate_authors};
use textmode::*;
use std::error::Error;

fn collect_sauce_from_options(values: Option<Values>) -> Vec<Sauce> {
    let mut vec = Vec::new();
    match values {
        Some(files) => {
            for file in files {
                match Sauce::from_file(file) {
                    Ok(sauce) => vec.push(sauce),
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        },
        None => {
            match Sauce::from_stdin() {
                Ok(sauce) => vec.push(sauce),
                Err(e) => eprintln!("{}", e),
            }
        },
    }
    vec
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT")
            .multiple(true)
            .min_values(1)
            .help("Sets the input file(s) to use, standard input is read if absent."))
        .arg(Arg::with_name("sauce")
            .short("s")
            .long("sauce")
            .help("Displays sauce information."))
        .arg(Arg::with_name("sauce_csv")
            .long("sauce-csv")
            .help("Displays sauce information in CSV format."))
        .arg(Arg::with_name("sauce_json")
            .long("sauce-json")
            .help("Displays sauce information in JSON format."))
        .arg(Arg::with_name("sauce_remove")
            .short("r")
            .long("sauce-remove")
            .help("Removes SAUCE record."));
        // .arg(Arg::with_name("sauce-auto")
        //     .long("auto")
        //     .help("Automatically insert a SAUCE record for non-textmode files."));
    let matches = app.get_matches();
    let values = matches.values_of("INPUT");
    if matches.is_present("sauce") {
        for sauce in collect_sauce_from_options(values) {
            println!("{}", sauce);
        }
    } else if matches.is_present("sauce_json") {
        let sauces = collect_sauce_from_options(values);
        let string = serde_json::to_string_pretty(&sauces)?;
        println!("{}", string);
    } else if matches.is_present("sauce_remove") {
        match values {
            Some(files) => {
                for file in files {
                    if let Err(e) = Sauce::remove_from_file(file) {
                        eprintln!("{}: {}", file, e);
                    }
                }
            },
            None => {
                if let Err(e) = Sauce::remove_from_stdin() {
                    eprintln!("{}", e);
                }
            },
        }
    } else if matches.is_present("sauce_csv") {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        for sauce in collect_sauce_from_options(values) {
            wtr.serialize(sauce)?;
        }
        wtr.flush()?;
    }
    Ok(())
}

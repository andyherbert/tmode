use clap::{App, Arg};
use clap::{crate_name, crate_version, crate_description, crate_authors};
use std::fs::File;
use std::io::{stdin, Read, BufReader};
use textmode::sauce;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT")
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
    let mut bytes: Vec<u8> = Vec::new();
    if let Some(input) = matches.value_of("INPUT") {
        let file = File::open(input)?;
        BufReader::new(file)
            .read_to_end(&mut bytes)?;
    } else {
        stdin().read_to_end(&mut bytes)?;
    }
    if matches.is_present("sauce") {
        if let Some(sauce) = sauce::parse_bytes(&bytes) {
            println!("{}", sauce);
        } else {
            println!("No SAUCE record found.");
        }
    }
    if matches.is_present("sauce_json") {
        if let Some(sauce) = sauce::parse_bytes(&bytes) {
            if let Ok(json) = sauce.to_json() {
                println!("{}", json);
            }
        } else {
            println!("No SAUCE record found.");
        }
    }
    Ok(())
}

use clap::{App, Arg, ArgMatches, Values};
use clap::{crate_name, crate_version, crate_description, crate_authors};
use textmode::*;
use std::error::Error;
use std::path::Path;

fn sauces_to_vec(values: Option<Values>) -> Vec<Sauce> {
    let mut vec = Vec::new();
    match values {
        Some(files) => {
            for file in files {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(sauce) = sauce {
                            vec.push(sauce);
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        },
        None => {
            match Sauce::from_stdin() {
                Ok(sauce) => {
                    if let Some(sauce) = sauce {
                        vec.push(sauce);
                    }
                },
                Err(e) => eprintln!("{}", e),
            }
        },
    }
    vec
}

pub fn sauce_options(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("sauce_remove") {
        match matches.values_of("INPUT") {
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
    }
    if matches.is_present("sauce_filetype") {
        match matches.values_of("INPUT") {
            Some(files) => {
                for file in files {
                    let path = Path::new(file);
                    if let Some(extension) = path.extension() {
                        if let Some(extension) = extension.to_str() {
                            match extension.to_uppercase().as_str() {
                                "GIF" => {
                                    match image::image_dimensions(file) {
                                        Ok((width, height)) => {
                                            match Sauce::from_file(file) {
                                                Ok(sauce) => {
                                                    let mut sauce = match sauce {
                                                        Some(sauce) => sauce,
                                                        None => Sauce::new(),
                                                    };
                                                    sauce.datatype = Some(textmode::DataType::Bitmap);
                                                    sauce.filetype = Some(textmode::FileType::GIF);
                                                    sauce.type_info_1 = width as usize;
                                                    sauce.type_info_2 = height as usize;
                                                    match sauce.add_to_file(file) {
                                                        Ok(_) => println!("{}: Addded filetype Bitmap/GIF ({}x{})", file, width, height),
                                                        Err(e) => eprintln!("{}: {}", file, e),
                                                    }
                                                },
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "IT" => {
                                    match Sauce::from_file(file) {
                                        Ok(sauce) => {
                                            let mut sauce = match sauce {
                                                Some(sauce) => sauce,
                                                None => Sauce::new(),
                                            };
                                            sauce.datatype = Some(textmode::DataType::Audio);
                                            sauce.filetype = Some(textmode::FileType::IT);
                                            match sauce.add_to_file(file) {
                                                Ok(_) => println!("{}: Addded filetype Audio/IT", file),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "JPG" | "JPEG "=> {
                                    match image::image_dimensions(file) {
                                        Ok((width, height)) => {
                                            match Sauce::from_file(file) {
                                                Ok(sauce) => {
                                                    let mut sauce = match sauce {
                                                        Some(sauce) => sauce,
                                                        None => Sauce::new(),
                                                    };
                                                    sauce.datatype = Some(textmode::DataType::Bitmap);
                                                    sauce.filetype = Some(textmode::FileType::JPG);
                                                    sauce.type_info_1 = width as usize;
                                                    sauce.type_info_2 = height as usize;
                                                    match sauce.add_to_file(file) {
                                                        Ok(_) => println!("{}: Addded filetype Bitmap/JPG ({}x{})", file, width, height),
                                                        Err(e) => eprintln!("{}: {}", file, e),
                                                    }
                                                },
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "MP4" => {
                                    match Sauce::from_file(file) {
                                        Ok(sauce) => {
                                            let mut sauce = match sauce {
                                                Some(sauce) => sauce,
                                                None => Sauce::new(),
                                            };
                                            sauce.datatype = Some(textmode::DataType::Bitmap);
                                            sauce.filetype = Some(textmode::FileType::MPG);
                                            match sauce.add_to_file(file) {
                                                Ok(_) => println!("{}: Addded filetype Bitmap/MPG", file),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "PNG" => {
                                    match image::image_dimensions(file) {
                                        Ok((width, height)) => {
                                            match Sauce::from_file(file) {
                                                Ok(sauce) => {
                                                    let mut sauce = match sauce {
                                                        Some(sauce) => sauce,
                                                        None => Sauce::new(),
                                                    };
                                                    sauce.datatype = Some(textmode::DataType::Bitmap);
                                                    sauce.filetype = Some(textmode::FileType::PNG);
                                                    sauce.type_info_1 = width as usize;
                                                    sauce.type_info_2 = height as usize;
                                                    match sauce.add_to_file(file) {
                                                        Ok(_) => println!("{}: Addded filetype Bitmap/PNG ({}x{})", file, width, height),
                                                        Err(e) => eprintln!("{}: {}", file, e),
                                                    }
                                                },
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "RIP" => {
                                    match Sauce::from_file(file) {
                                        Ok(sauce) => {
                                            let mut sauce = match sauce {
                                                Some(sauce) => sauce,
                                                None => Sauce::new(),
                                            };
                                            sauce.datatype = Some(textmode::DataType::Character);
                                            sauce.filetype = Some(textmode::FileType::RIPScript);
                                            match sauce.add_to_file(file) {
                                                Ok(_) => println!("{}: Addded filetype Character/RIPScript", file),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                "S3M" => {
                                    match Sauce::from_file(file) {
                                        Ok(sauce) => {
                                            let mut sauce = match sauce {
                                                Some(sauce) => sauce,
                                                None => Sauce::new(),
                                            };
                                            sauce.datatype = Some(textmode::DataType::Audio);
                                            sauce.filetype = Some(textmode::FileType::S3M);
                                            match sauce.add_to_file(file) {
                                                Ok(_) => println!("{}: Addded filetype Audio/S3M", file),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                _ => {},
                            }
                        }
                    }
                }
            },
            None => eprintln!("Error: filetype switch does not work with stdin input"),
        }
    }
    if matches.is_present("sauce") {
        match matches.values_of("INPUT") {
            Some(files) => {
                for file in files {
                    match Sauce::from_file(file) {
                        Ok(sauce) => {
                            if let Some(sauce) = sauce {
                                println!("{}", sauce);
                            }
                        },
                        Err(e) => eprintln!("{}: {}", file, e),
                    }
                }
            },
            None => {
                match Sauce::from_stdin() {
                    Ok(sauce) => {
                        if let Some(sauce) = sauce {
                            println!("{}", sauce);
                        }
                    },
                    Err(e) => eprintln!("{}", e),
                }
            },
        }
    }
    if matches.is_present("sauce_csv") {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        for sauce in sauces_to_vec(matches.values_of("INPUT")) {
            wtr.serialize(sauce)?;
        }
        wtr.flush()?;
    }
    if matches.is_present("sauce_json") {
        let vec = sauces_to_vec(matches.values_of("INPUT"));
        let string = serde_json::to_string_pretty(&vec)?;
        println!("{}", string);
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
        .help("Sets the input file(s) to use, standard input is read if absent.")
    ).arg(Arg::with_name("sauce")
        .short("s")
        .long("sauce")
        .help("Displays sauce information."))
    .arg(Arg::with_name("sauce_csv")
        .long("sauce-csv")
        .help("Displays sauce information in CSV format."))
    .arg(Arg::with_name("sauce_filetype")
        .long("sauce-filetype")
        .help("Automatically insert a SAUCE record for non-textmode files."))
    .arg(Arg::with_name("sauce_json")
        .long("sauce-json")
        .help("Displays sauce information in JSON format."))
    .arg(Arg::with_name("sauce_remove")
        .short("r")
        .long("sauce-remove")
        .help("Removes SAUCE record."));
    let matches = app.get_matches();
    sauce_options(&matches)?;
    Ok(())
}

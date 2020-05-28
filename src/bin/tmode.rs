use clap::{App, Arg, ArgMatches, crate_name, crate_version, crate_description, crate_authors};
use textmode::*;
use std::error::Error;
use std::path::Path;
use chrono::{Datelike, Local};
use std::fs;

pub fn sauce_options(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("sauce_remove") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
            if let Err(e) = Sauce::remove_from_file(file) {
                eprintln!("{}: {}", file, e);
            }
        }
    }
    if matches.is_present("filetype") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
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
                                                Ok(_) => println!("{}: Added filetype Bitmap/GIF ({}x{})", file, width, height),
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
                                        Ok(_) => println!("{}: Added filetype Audio/IT", file),
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
                                                Ok(_) => println!("{}: Added filetype Bitmap/JPG ({}x{})", file, width, height),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                Err(_) => {
                                    match Sauce::from_file(file) {
                                        Ok(sauce) => {
                                            let mut sauce = match sauce {
                                                Some(sauce) => sauce,
                                                None => Sauce::new(),
                                            };
                                            sauce.datatype = Some(textmode::DataType::Bitmap);
                                            sauce.filetype = Some(textmode::FileType::JPG);
                                            match sauce.add_to_file(file) {
                                                Ok(_) => println!("{}: Added filetype Bitmap/JPG", file),
                                                Err(e) => eprintln!("{}: {}", file, e),
                                            }
                                        },
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
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
                                        Ok(_) => println!("{}: Added filetype Bitmap/MPG", file),
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
                                                Ok(_) => println!("{}: Added filetype Bitmap/PNG ({}x{})", file, width, height),
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
                                        Ok(_) => println!("{}: Added filetype Character/RIPScript", file),
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
                                        Ok(_) => println!("{}: Added filetype Audio/S3M", file),
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
    }
    if matches.is_present("title") {
        if let Some(title) = matches.value_of("title") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            sauce.title = title.to_string();
                            match sauce.add_to_file(file) {
                                Ok(_) => println!("{}: Added title", file),
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("author") {
        if let Some(author) = matches.value_of("author") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            sauce.author = author.to_string();
                            match sauce.add_to_file(file) {
                                Ok(_) => println!("{}: Added author", file),
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("group") {
        if let Some(group) = matches.value_of("group") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            sauce.group = group.to_string();
                            match sauce.add_to_file(file) {
                                Ok(_) => println!("{}: Added group", file),
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("current_date") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
            match Sauce::from_file(file) {
                Ok(sauce) => {
                    if let Some(mut sauce) = sauce {
                        let time = Local::now();
                        let (_, year) = time.year_ce();
                        let month = time.month();
                        let day = time.day();
                        sauce.year = format!("{:02}", year);
                        sauce.month = format!("{:02}", month);
                        sauce.date = format!("{:02}", day);
                        match sauce.add_to_file(file) {
                            Ok(_) => println!("{}: Added current date", file),
                            Err(e) => eprintln!("{}: {}", file, e),
                        }
                    }
                },
                Err(e) => eprintln!("{}: {}", file, e),
            }
        }
    }
    if matches.is_present("year") {
        if let Some(year) = matches.value_of("year") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            match year.parse::<usize>() {
                                Ok(year) => {
                                    sauce.year = format!("{:04}", year);
                                    match sauce.add_to_file(file) {
                                        Ok(_) => println!("{}: Added year", file),
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("month") {
        if let Some(month) = matches.value_of("month") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            match month.parse::<usize>() {
                                Ok(month) => {
                                    sauce.month = format!("{:02}", month);
                                    match sauce.add_to_file(file) {
                                        Ok(_) => println!("{}: Added month", file),
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("date") {
        if let Some(date) = matches.value_of("date") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            match date.parse::<usize>() {
                                Ok(date) => {
                                    sauce.date = format!("{:02}", date);
                                    match sauce.add_to_file(file) {
                                        Ok(_) => println!("{}: Added date", file),
                                        Err(e) => eprintln!("{}: {}", file, e),
                                    }
                                },
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("font") {
        if let Some(font) = matches.value_of("font") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            sauce.font_name = font.to_string();
                            match sauce.add_to_file(file) {
                                Ok(_) => println!("{}: Added font {}", file, font),
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("comments") {
        if let Some(comments) = matches.value_of("comments") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            if comments.is_empty() {
                                sauce.comments = None;
                            } else {
                                sauce.comments = Some(comments.to_string());
                            }
                            match sauce.add_to_file(file) {
                                Ok(_) => println!("{}: Added comments", file),
                                Err(e) => eprintln!("{}: {}", file, e),
                            }
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
        }
    }
    if matches.is_present("no_sauce") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
            match Sauce::from_file(file) {
                Ok(sauce) => {
                    if let None = sauce {
                        println!("{}: No SAUCE record found", file);
                    }
                },
                Err(e) => eprintln!("{}: {}", file, e),
            }
        }
    }
    if matches.is_present("sauce") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
            match Sauce::from_file(file) {
                Ok(sauce) => {
                    if let Some(sauce) = sauce {
                        println!("{}", sauce);
                    }
                },
                Err(e) => eprintln!("{}: {}", file, e),
            }
        }
    }
    if matches.is_present("csv_file") {
        if let Some(file) = matches.value_of("csv_file") {
            let mut wtr = csv::Writer::from_path(file)?;
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(sauce) = sauce {
                            wtr.serialize(sauce)?;
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
            wtr.flush()?;
        }
    }
    if matches.is_present("json_file") {
        if let Some(file) = matches.value_of("json_file") {
            let mut vec = Vec::new();
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(sauce) = sauce {
                            vec.push(sauce);
                        }
                    },
                    Err(e) => eprintln!("{}: {}", file, e),
                }
            }
            let file = fs::File::create(file)?;
            serde_json::to_writer_pretty(&file, &vec)?;
        }
    }
    if matches.is_present("import_json") {
        if let Some(file) = matches.value_of("import_json") {
            let json = fs::read_to_string(file)?;
            let sauces: Vec<Sauce> = serde_json::from_str(&json)?;
            for mut sauce in sauces {
                if let Some(file) = sauce.file.clone() {
                    match sauce.add_to_file(&file) {
                        Ok(_) => println!("{}: Updated", file),
                        Err(e) => eprintln!("{}: {}", file, e),
                    }
                }
            }
        }
    }
    if matches.is_present("import_csv") {
        if let Some(file) = matches.value_of("import_csv") {
            let mut rdr = csv::Reader::from_path(file)?;
            for result in rdr.deserialize() {
                let mut sauce: Sauce = result?;
                if let Some(file) = sauce.file.clone() {
                    match sauce.add_to_file(&file) {
                        Ok(_) => println!("{}: Updated", file),
                        Err(e) => eprintln!("{}: {}", file, e),
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
    .arg(Arg::with_name("FILE")
        .multiple(true)
        .min_values(1)
        .help("Sets the input file(s) to use."))
    .arg(Arg::with_name("sauce")
        .short("s")
        .long("sauce")
        .requires("FILE")
        .help("Displays SAUCE information."))
    .arg(Arg::with_name("csv_file")
        .long("csv")
        .takes_value(true)
        .requires("FILE")
        .help("Writes SAUCE information to a CSV file."))
    .arg(Arg::with_name("import_csv")
        .long("import-csv")
        .takes_value(true)
        .help("Writes a CSV file with multiple SAUCE records."))
    .arg(Arg::with_name("filetype")
        .long("filetype")
        .requires("FILE")
        .help("Automatically insert a SAUCE record for non-textmode files."))
    .arg(Arg::with_name("json_file")
        .long("json")
        .takes_value(true)
        .requires("FILE")
        .help("Writes a JSON file with multiple SAUCE records."))
    .arg(Arg::with_name("import_json")
        .long("import-json")
        .takes_value(true)
        .help("Reads a JSON file to update multiple SAUCE records."))
    .arg(Arg::with_name("sauce_remove")
        .short("r")
        .long("sauce-remove")
        .requires("FILE")
        .help("Removes SAUCE record."))
    .arg(Arg::with_name("title")
        .long("title")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a title to the SAUCE record."))
    .arg(Arg::with_name("author")
        .long("author")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a author to the SAUCE record."))
    .arg(Arg::with_name("group")
        .long("group")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a group to the SAUCE record."))
    .arg(Arg::with_name("year")
        .long("year")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a year to the SAUCE record."))
    .arg(Arg::with_name("month")
        .long("month")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a month to the SAUCE record."))
    .arg(Arg::with_name("date")
        .long("date")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a date of the month to the SAUCE record."))
    .arg(Arg::with_name("current_date")
        .long("current-date")
        .requires("FILE")
        .help("Adds the current date to the SAUCE record."))
    .arg(Arg::with_name("font")
        .long("font")
        .takes_value(true)
        .requires("FILE")
        .help("Adds a font name to the SAUCE record."))
    .arg(Arg::with_name("comments")
        .long("comments")
        .takes_value(true)
        .requires("FILE")
        .help("Adds comments to the SAUCE record."))
    .arg(Arg::with_name("no_sauce")
        .long("no-sauce")
        .requires("FILE")
        .help("Lists all the files with no SAUCE record."));
    let matches = app.get_matches();
    sauce_options(&matches)?;
    Ok(())
}

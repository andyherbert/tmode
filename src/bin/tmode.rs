use clap::{clap_app, ArgMatches, crate_version, crate_description, crate_authors};
use std::u64;
use textmode::*;
use std::error::Error;
use std::path::Path;
use chrono::{Datelike, Local};
use std::fs;

pub fn sauce_options(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("sauce_remove") {
        for file in matches.values_of("FILE").expect("Error: No input files") {
            match Sauce::remove_from_file(file) {
                Ok(()) => println!("{}: SAUCE record removed", file),
                Err(e) => eprintln!("{}: {}", file, e),
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
    if matches.is_present("day") {
        if let Some(date) = matches.value_of("day") {
            for file in matches.values_of("FILE").expect("Error: No input files") {
                match Sauce::from_file(file) {
                    Ok(sauce) => {
                        if let Some(mut sauce) = sauce {
                            match date.parse::<usize>() {
                                Ok(date) => {
                                    sauce.date = format!("{:02}", date);
                                    match sauce.add_to_file(file) {
                                        Ok(_) => println!("{}: Added day", file),
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
    if matches.is_present("sauce_display") {
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
    if matches.is_present("export_csv") {
        if let Some(file) = matches.value_of("export_csv") {
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
    if matches.is_present("export_json") {
        if let Some(file) = matches.value_of("export_json") {
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
    let matches = app.get_matches();
    if let Err(e) = sauce_options(&matches) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

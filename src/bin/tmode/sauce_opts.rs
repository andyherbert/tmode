use textmode::{Sauce, sauce::DataType, sauce::FileType};
use std::error::Error;
use std::path::Path;
use chrono::{Datelike, Local};
use std::fs;
use clap::{ArgMatches, Values};
use image::image_dimensions;
use fs::File;

fn sauce_remove(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    for file in values.expect("Error: No input files") {
        match Sauce::remove_from_file(file) {
            Ok(()) => println!("{}: SAUCE record removed", file),
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn filetype(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    for file in values.expect("Error: No input files") {
        let path = Path::new(file);
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                match extension.to_uppercase().as_str() {
                    "GIF" => {
                        match image_dimensions(file) {
                            Ok((width, height)) => {
                                match Sauce::from_file(file) {
                                    Ok(sauce) => {
                                        let mut sauce = match sauce {
                                            Some(sauce) => sauce,
                                            None => Sauce::new(),
                                        };
                                        sauce.datatype = Some(DataType::Bitmap);
                                        sauce.filetype = Some(FileType::GIF);
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
                                sauce.datatype = Some(DataType::Audio);
                                sauce.filetype = Some(FileType::IT);
                                match sauce.add_to_file(file) {
                                    Ok(_) => println!("{}: Added filetype Audio/IT", file),
                                    Err(e) => eprintln!("{}: {}", file, e),
                                }
                            },
                            Err(e) => eprintln!("{}: {}", file, e),
                        }
                    },
                    "JPG" | "JPEG "=> {
                        match image_dimensions(file) {
                            Ok((width, height)) => {
                                match Sauce::from_file(file) {
                                    Ok(sauce) => {
                                        let mut sauce = match sauce {
                                            Some(sauce) => sauce,
                                            None => Sauce::new(),
                                        };
                                        sauce.datatype = Some(DataType::Bitmap);
                                        sauce.filetype = Some(FileType::JPG);
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
                                        sauce.datatype = Some(DataType::Bitmap);
                                        sauce.filetype = Some(FileType::JPG);
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
                                sauce.datatype = Some(DataType::Bitmap);
                                sauce.filetype = Some(FileType::MPG);
                                match sauce.add_to_file(file) {
                                    Ok(_) => println!("{}: Added filetype Bitmap/MPG", file),
                                    Err(e) => eprintln!("{}: {}", file, e),
                                }
                            },
                            Err(e) => eprintln!("{}: {}", file, e),
                        }
                    },
                    "PNG" => {
                        match image_dimensions(file) {
                            Ok((width, height)) => {
                                match Sauce::from_file(file) {
                                    Ok(sauce) => {
                                        let mut sauce = match sauce {
                                            Some(sauce) => sauce,
                                            None => Sauce::new(),
                                        };
                                        sauce.datatype = Some(DataType::Bitmap);
                                        sauce.filetype = Some(FileType::PNG);
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
                                sauce.datatype = Some(DataType::Character);
                                sauce.filetype = Some(FileType::RIPScript);
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
                                sauce.datatype = Some(DataType::Audio);
                                sauce.filetype = Some(FileType::S3M);
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
    Ok(())
}

fn title(values: Option<Values>, title: Option<&str>) -> Result<(), Box<dyn Error>> {
    let title = title.expect("No title given");
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn author(values: Option<Values>, author: Option<&str>) -> Result<(), Box<dyn Error>> {
    let author = author.expect("No author given");
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn group(values: Option<Values>, group: Option<&str>) -> Result<(), Box<dyn Error>> {
    let group = group.expect("No group given");
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn current_date(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn year(values: Option<Values>, year: Option<&str>) -> Result<(), Box<dyn Error>> {
    let year = year.expect("No year given");
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn month(values: Option<Values>, month: Option<&str>) -> Result<(), Box<dyn Error>> {
    let month = month.expect("No month given");
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn day(values: Option<Values>, day: Option<&str>) -> Result<(), Box<dyn Error>> {
    let day = day.expect("No day given");
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let Some(mut sauce) = sauce {
                    match day.parse::<usize>() {
                        Ok(day) => {
                            sauce.date = format!("{:02}", day);
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
    Ok(())
}

fn font(values: Option<Values>, font: Option<&str>) -> Result<(), Box<dyn Error>> {
    let font = font.expect("No font given");
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let Some(mut sauce) = sauce {
                    sauce.font_name = font.to_string();
                    match sauce.add_to_file(file) {
                        Ok(_) => println!("{}: Added font", file),
                        Err(e) => eprintln!("{}: {}", file, e),
                    }
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn comments(values: Option<Values>, comments: Option<&str>) -> Result<(), Box<dyn Error>> {
    let comments = comments.expect("No comments given");
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let Some(mut sauce) = sauce {
                    sauce.comments = if comments.is_empty() {
                        None
                    } else {
                        Some(comments.to_string())
                    };
                    match sauce.add_to_file(file) {
                        Ok(_) => println!("{}: Added comments", file),
                        Err(e) => eprintln!("{}: {}", file, e),
                    }
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn no_sauce(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let None = sauce {
                    println!("{}: No SAUCE record found", file);
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn sauce_display(values: Option<Values>) -> Result<(), Box<dyn Error>> {
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let Some(sauce) = sauce {
                    println!("{}", sauce);
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn export_csv(values: Option<Values>, csv_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let csv_file = csv_file.expect("No csv file given");
    let mut wtr = csv::Writer::from_path(csv_file)?;
    for file in values.expect("Error: No input files") {
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
    Ok(())
}

fn export_json(values: Option<Values>, json_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let json_file = json_file.expect("No json file given");
    let mut vec = Vec::new();
    for file in values.expect("Error: No input files") {
        match Sauce::from_file(file) {
            Ok(sauce) => {
                if let Some(sauce) = sauce {
                    vec.push(sauce);
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    let file = File::create(json_file)?;
    serde_json::to_writer_pretty(file, &vec)?;
    Ok(())
}

fn import_csv(file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let file = file.expect("No csv file given");
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
    Ok(())
}

fn import_json(file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let file = file.expect("No json file given");
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
    Ok(())
}

pub fn sauce_options(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if matches.is_present("sauce_remove") {
        sauce_remove(matches.values_of("FILE"))?;
    }
    if matches.is_present("filetype") {
        filetype(matches.values_of("FILE"))?;
    }
    if matches.is_present("title") {
        title(matches.values_of("FILE"), matches.value_of("title"))?;
    }
    if matches.is_present("author") {
        author(matches.values_of("FILE"), matches.value_of("author"))?;
    }
    if matches.is_present("group") {
        group(matches.values_of("FILE"), matches.value_of("group"))?;
    }
    if matches.is_present("current_date") {
        current_date(matches.values_of("FILE"))?;
    }
    if matches.is_present("year") {
        year(matches.values_of("FILE"), matches.value_of("year"))?;
    }
    if matches.is_present("month") {
        month(matches.values_of("FILE"), matches.value_of("month"))?;
    }
    if matches.is_present("day") {
        day(matches.values_of("FILE"), matches.value_of("day"))?;
    }
    if matches.is_present("font") {
        font(matches.values_of("FILE"), matches.value_of("font"))?;
    }
    if matches.is_present("comments") {
        comments(matches.values_of("FILE"), matches.value_of("comments"))?;
    }
    if matches.is_present("no_sauce") {
        no_sauce(matches.values_of("FILE"))?;
    }
    if matches.is_present("sauce_display") {
        sauce_display(matches.values_of("FILE"))?;
    }
    if matches.is_present("export_csv") {
        export_csv(matches.values_of("FILE"), matches.value_of("export_csv"))?;
    }
    if matches.is_present("export_json") {
        export_json(matches.values_of("FILE"), matches.value_of("export_json"))?;
    }
    if matches.is_present("import_csv") {
        import_csv(matches.value_of("import_csv"))?;
    }
    if matches.is_present("import_json") {
        import_json(matches.value_of("import_json"))?;
    }
    Ok(())
}

# Textmode.rs

Textmode art interpretor, library, and tools targeting native code execution, the web, and WASI.

To build install [Rust and Cargo](https://www.rust-lang.org/tools/install), then:

`cargo build`

To run:

`cargo run -- <command line options>`

```
tmode 0.0.1
Andy Herbert <andy.herbert@gmail.com>
Textmode art interpretor, library, and tools.

USAGE:
    tmode [FLAGS] [OPTIONS] [FILE]...

FLAGS:
        --current-date            Adds the current local date to SAUCE records.
        --filetype                Automatically insert a SAUCE record for non-textmode files.
    -h, --help                    Prints help information
        --ice-colors              Enables iCE colors for supported filetypes.
        --legacy-aspect           Enables legacy aspect ratio for supported filetypes.
        --letter-spacing          Enables 9px fonts for supported filetypes.
        --modern-aspect           Enables modern aspect ratio for supported filetypes.
        --no-ice-colors           Disables iCE colors for supported filetypes.
        --no-letter-spacing       Disables 9px fonts for supported filetypes.
        --no-sauce                Lists all the files with no SAUCE record.
    -s, --sauce                   Displays SAUCE information.
    -r, --remove                  Removes SAUCE records.
        --unset-aspect            Sets no preference on aspect ratio setting for supported filetypes.
        --unset-letter-spacing    Sets no preference on letter spacing for supported filetypes.
    -V, --version                 Prints version information

OPTIONS:
        --author=<author's name>       Adds an author to SAUCE records.
        --comments=<comments>          Adds comments to SAUCE records.
        --day=<day of the month>       Adds a day of the month to SAUCE records.
        --export-csv=<CSV FILE>        Exports multiple SAUCE records to a CSV file.
        --export-json=<JSON FILE>      Exports multiple SAUCE records to a JSON file.
        --font=<font name>             Adds a font name to SAUCE records.
        --group=<group's name>         Adds a group to SAUCE records.
        --import-csv=<CSV FILE>        Imports a CSV file to update multiple SAUCE records.
        --import-json=<JSON FILE>      Imports a JSON file to update multiple SAUCE records.
        --info-1=<numeric value>       Sets a numeric value to information 1.
        --info-2=<numeric value>       Sets a numeric value to information 2.
        --info-3=<numeric value>       Sets a numeric value to information 3.
        --info-4=<numeric value>       Sets a numeric value to information 4.
        --month=<month of the year>    Adds a (numeric) month of the year to SAUCE records.
        --title=<title>                Adds a title to the SAUCE records.
        --year=<year>                  Adds a year to SAUCE records.

ARGS:
    <FILE>...    Sets the input file(s) to use.
```

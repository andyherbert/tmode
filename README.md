# Textmode.rs

Textmode art interpretor, library, and tools targeting native code execution, the web, and WASI.

```
textmode 0.0.1
Andy Herbert <andy.herbert@gmail.com>
Textmode art interpretor, library, and tools.

USAGE:
    tmode [FLAGS] [OPTIONS] [FILE]...

FLAGS:
        --current-date    Adds the current date to the SAUCE record.
        --filetype        Automatically insert a SAUCE record for non-textmode files.
    -h, --help            Prints help information
        --no-sauce        Lists all the files with no SAUCE record.
    -s, --sauce           Displays SAUCE information.
    -r, --sauce-remove    Removes SAUCE record.
    -V, --version         Prints version information

OPTIONS:
        --author <author>              Adds a author to the SAUCE record.
        --comments <comments>          Adds comments to the SAUCE record.
        --csv <csv_file>               Writes SAUCE information to a CSV file.
        --date <date>                  Adds a date of the month to the SAUCE record.
        --font <font>                  Adds a font name to the SAUCE record.
        --group <group>                Adds a group to the SAUCE record.
        --import-csv <import_csv>      Writes a CSV file with multiple SAUCE records.
        --import-json <import_json>    Reads a JSON file to update multiple SAUCE records.
        --json <json_file>             Writes a JSON file with multiple SAUCE records.
        --month <month>                Adds a month to the SAUCE record.
        --title <title>                Adds a title to the SAUCE record.
        --year <year>                  Adds a year to the SAUCE record.

ARGS:
    <FILE>...    Sets the input file(s) to use.
```
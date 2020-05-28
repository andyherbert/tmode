# Textmode.rs

Textmode art interpretor, library, and tools targeting native code execution, the web, and WASI.

```
textmode 0.0.1
Andy Herbert <andy.herbert@gmail.com>
Textmode art interpretor, library, and tools.

USAGE:
    tmode [FLAGS] [OPTIONS] [FILE]...

FLAGS:
        --current-date    Adds the current local date to SAUCE records.
        --filetype        Automatically insert a SAUCE record for non-textmode files.
    -h, --help            Prints help information
        --no-sauce        Lists all the files with no SAUCE record.
    -s, --sauce           Displays SAUCE information.
    -r, --sauce-remove    Removes SAUCE records.
    -V, --version         Prints version information

OPTIONS:
        --author=<author's name>     Adds an author to SAUCE records.
        --comments=<comments>        Adds comments to SAUCE records.
        --day=<day of month>         Adds a day of the month to SAUCE records.
        --export-csv=<CSV FILE>      Exports multiple SAUCE records to a CSV file.
        --export-json=<JSON FILE>    Exports multiple SAUCE records to a JSON file.
        --font=<font name>           Adds a font name to SAUCE records.
        --group=<group's name>       Adds a group to SAUCE records.
        --import-csv=<CSV FILE>      Imports a CSV file to update multiple SAUCE records.
        --import-json=<JSON FILE>    Imports a JSON file to update multiple SAUCE records.
        --month=<month>              Adds a month to SAUCE records.
        --title=<title>              Adds a title to the SAUCE records.
        --year=<year>                Adds a year to SAUCE records.

ARGS:
    <FILE>...    Sets the input file(s) to use.
```
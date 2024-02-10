# Notes

Write notes from command line. Just my Rust practice project. If you are a more experienced
Rust developer feel free to propose improvements.

### Usage

Without any params, it will print all the notes:

`notes`

To add a note (but keep in mind limitations of your shell):

`notes Hello, remember to avoid or escape special characters`

To edit notes (stored in `~/.notes`):

`notes -e` or `notes --edit`

For help:

`notes -h` or `notes --help`

### Things that could be improved

- Parser
- Command matching

Any suggestions are welcome.


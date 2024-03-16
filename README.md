# PRSR

PRSR is a simple command line parser written in Rust.

## Features

- Parses markdown files and outputs the parsed content.
- Option to wrap the output in HTML.
- Option to apply a CSS file to the HTML output.
- Prints parsing events if requested.

## Usage

```sh
cargo run -- [FLAGS] [OPTIONS] --input <input>
```

## Flags

- -e, --event: Print parsing events.
- -w, --wrap: Wrap output in HTML.

## Options

- --css <css>: Path to a CSS file to apply to the HTML output.
- --input <input>: Sets the input file to use.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

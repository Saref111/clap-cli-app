use std::path::PathBuf;

use clap::Parser;
use pulldown_cmark::{html::push_html, Event, Parser as MDParser};

#[derive(Parser)]
#[command(name = "CLI-MD-parser")]
#[command(version, about, author)]
struct Cli {
    /// Sets the input file
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file_string = std::fs::read_to_string(cli.input);
    let file_string = file_string.unwrap();
    println!("{file_string}");
}

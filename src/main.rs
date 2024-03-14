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
    let file_string = std::fs::read_to_string(cli.input).expect("Could not read the file");
    let parser = MDParser::new(&file_string);
    let parser_iter: Vec<Event> = parser.into_iter().collect();
    let mut file_md = String::new();

    for evt in &parser_iter {
        println!("{:?}", evt);
    }

    push_html(&mut file_md, parser_iter.into_iter());

    println!("{file_md}");
}

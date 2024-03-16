use std::path::PathBuf;

use clap::Parser;
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser as MDParser};

#[cfg(test)]
mod test;

#[derive(Parser)]
#[command(name = "CLI-MD-parser")]
#[command(version, about, author)]
struct Cli {
    /// Sets the input file
    input: PathBuf,

    /// Wrap in html
    #[arg(long, short)]
    wrap: bool,
    /// Css path
    #[arg(long)]
    css: Option<String>,
    /// Print parsing events
    #[arg(long, short)]
    event: bool,
}

fn wrap_html(string: &str, css: Option<&str>) -> String {
    let res = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf8";
                @if let Some(s) = css {
                    link rel="stylesheet" type="text/css" href=(s);
                }
            }
            body {
                (maud::PreEscaped(string))
            }
        }
    };

    return res.into_string();
}

fn main() {
    let cli = Cli::parse();
    let file_string = std::fs::read_to_string(cli.input).expect("Could not read the file");
    let parser = MDParser::new(&file_string);
    let parser_iter: Vec<Event> = parser.into_iter().collect();
    let mut file_md = String::new();

    if cli.event {
        for evt in &parser_iter {
            println!("{:?}", evt);
        }
    }
    push_html(&mut file_md, parser_iter.into_iter());

    if cli.wrap {
        file_md = wrap_html(&file_md, cli.css.as_deref().map(|s| s));
    }

    println!("{file_md}");
}

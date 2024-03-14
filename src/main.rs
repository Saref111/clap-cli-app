use clap::Parser;

#[derive(Parser)]
#[command(name = "CLI-parser")]
#[command(version, about)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(short, long)]
    one: Option<String>,
    #[arg(short, long)]
    two: String,
}

fn main() {
    let cli = Cli::parse();

    println!("One: {:?}", cli.one);
    println!("Two: {}", cli.two);
}

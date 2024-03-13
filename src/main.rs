use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "char", about = "Prints a character")]
struct Args {
    #[clap(short, long)]
    char: char,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.char);
}

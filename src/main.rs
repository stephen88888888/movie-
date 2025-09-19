use clap::Parser;
use std::error;

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
//这是我的代

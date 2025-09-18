use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}

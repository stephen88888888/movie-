use clap::{Parser, Subcommand};
use movie::handler::handle_login;

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///user log into the system
    Login {
        ///The username of the user
        #[arg(short, long)]
        username: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handle_login(username)?,
        _ => println!("No command provided or command nto recognized"),
    }

    Ok(())
}


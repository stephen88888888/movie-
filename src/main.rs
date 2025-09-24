use clap::{Parser, Subcommand};
use movie::handler::{handle_login, handle_logout};

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// User log into the system
    Login {
        /// The username of the user
        #[arg(short, long)]
        username: String,
    },
    /// Log out
    Logout,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库连接
    movie::database::init().await?;
    
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => {
            handle_login(username).await?;
        }
        Some(Commands::Logout) => {
            handle_logout().await?;
        }
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }

    Ok(())
}
// 导入命令行参数解析库和子命令支持
use clap::{Parser, Subcommand};
// 导入处理登录和退出的函数
use movie::handler::{handle_login, handle_logout};

// 定义主命令行接口结构
#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {
    // 定义子命令字段
    #[command(subcommand)]
    commands: Option<Commands>,
}

// 定义可用的子命令枚举
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

// 主函数，使用tokio异步运行时
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库连接
    movie::database::init().await?;

    // 解析命令行参数
    let cli = Cli::parse();

    // 根据不同的命令执行相应的处理逻辑
    match &cli.commands {
        Some(Commands::Login { username }) => {
            // 处理登录命令
            handle_login(username).await?;
        }
        Some(Commands::Logout) => {
            // 处理退出命令
            handle_logout().await?;
        }
        None => {
            // 没有提供命令时的提示信息
            println!("No command provided. Use --help for usage information.");
        }
    }

    // 程序正常结束
    Ok(())
}

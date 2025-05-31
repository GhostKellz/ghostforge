use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "GhostForge — a makepkg replacement", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Install,
    Lint,
    Sign,
    Publish,
    Info,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => println!("🔧 Building package..."),
        Commands::Install => println!("📦 Installing package..."),
        Commands::Lint => println!("🧪 Linting package metadata..."),
        Commands::Sign => println!("🔐 Signing package..."),
        Commands::Publish => println!("🚀 Publishing package..."),
        Commands::Info => println!("📄 Showing package info..."),
    }
}


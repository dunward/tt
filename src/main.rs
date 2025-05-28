use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tt", version, author, about = "AI-based terminal command helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            show_status();
        }
    }
}

fn show_status() {
    use std::env;
    use std::path::PathBuf;

    let cwd: PathBuf = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let user = env::var("USERNAME").or_else(|_| env::var("USER")).unwrap_or_else(|_| "Unknown".into());

    println!("User: {}", user);
    println!("Current directory: {}", cwd.display());
}

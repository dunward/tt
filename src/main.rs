use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use std::env;
use tokio;

#[derive(Parser)]
#[command(name = "tt", version, author, about = "AI-based terminal command helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ask AI for command suggestions
    Ask {
        #[arg(required = true, help = "Natural language query")]
        query: String,
    },
    /// Show current status
    Status,
    /// Configure AI settings
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { query } => {
            if let Err(e) = ask_ai(query).await {
                error!("Failed to ask AI: {}", e);
                return Err(e);
            }
        }
        Commands::Status => {
            show_status();
        }
        Commands::Config => {
            config();
        }
    }

    Ok(())
}

async fn ask_ai(query: String) -> Result<()> {
    // TODO: Implement AI communication
    // This will be implemented with actual LLM API integration
    info!("Asking AI: {}", query);
    Ok(())
}

fn show_status() {
    let cwd = env::current_dir().unwrap_or_else(|_| "unknown directory".into());
    let user = env::var("USERNAME").or_else(|_| env::var("USER")).unwrap_or_else(|_| "Unknown".into());

    println!("User: {}", user);
    println!("Current directory: {}", cwd.display());
}

fn config() {
    // TODO: Implement configuration management
    println!("Configuration management will be implemented here");
}

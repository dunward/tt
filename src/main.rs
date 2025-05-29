use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use std::env;
use tokio;
use dirs;
use serde_json;

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
    Config(Configure),
}

#[derive(Parser)]
struct Configure {
    #[command(subcommand)]
    config_command: ConfigCommand,
}

#[derive(Subcommand)]
enum ConfigCommand {
    /// Configure OpenAI API settings
    OpenAI {
        #[arg(required = true, help = "OpenAI API key")]
        api_key: String,
    },
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
        Commands::Config(config) => {
            match config.config_command {
                ConfigCommand::OpenAI { api_key } => {
                    if let Err(e) = configure_openai(api_key) {
                        error!("Failed to configure OpenAI: {}", e);
                        return Err(e);
                    }
                }
            }
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

fn configure_openai(api_key: String) -> Result<()> {
    use std::fs;
    use std::path::Path;
    use serde_json::json;

    let config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;
    let config_file = config_dir.join("tt").join("config.json");

    // Create config directory if it doesn't exist
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // Read existing config or create new one
    let config = if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::Map::new()
    };

    // Update config with OpenAI API key
    let mut config = config.as_object_mut().unwrap_or(&mut serde_json::Map::new());
    config.insert("openai_api_key".to_string(), json!(api_key));

    // Write config back to file
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(config_file, config_str)?;

    info!("OpenAI API key configured successfully");
    Ok(())
}

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use std::env;
use tokio;
use dirs;
use serde_json;
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt, Pid, PidExt};

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

// Check if a specific API key is configured
fn is_key_configured(key_name: &str) -> String {
    let config_dir = dirs::config_dir().unwrap_or_else(|| "unknown config dir".into());
    let config_file = config_dir.join("tt").join("config.json");

    match fs::read_to_string(&config_file) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(config) => config.get(key_name).map_or("No", |key| 
                if key.is_string() && !key.as_str().unwrap_or_default().is_empty() {
                    "Yes"
                } else {
                    "No"
                }
            ).to_string(),
            Err(_) => "No".to_string(),
        },
        Err(_) => "No".to_string(),
    }
}

fn show_status() {
    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );

    let parent_name = sysinfo::get_current_pid()
        .ok()
        .and_then(|pid| system.process(pid))
        .and_then(|proc| proc.parent())
        .and_then(|parent_pid| system.process(parent_pid))
        .map(|proc| proc.name())
        .unwrap_or("Unknown shell");

    println!("Current shell: {}", parent_name);
    println!("OpenAI API Key Configured: {}", is_key_configured("openai_api_key"));
}

fn config() {
    // TODO: Implement configuration management
    println!("Configuration management will be implemented here");
}

fn configure_openai(api_key: String) -> Result<()> {
    use std::fs;
    use serde_json;

    let config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;
    let config_file = config_dir.join("tt").join("config.json");

    // Create config directory if it doesn't exist
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // Read existing config or create new one
    let config = if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        serde_json::from_str::<serde_json::Value>(&content)?
    } else {
        serde_json::json!({ "openai_api_key": api_key })
    };

    // Write config back to file
    let config_str = serde_json::to_string_pretty(&config)?;
    fs::write(config_file, config_str)?;

    info!("OpenAI API key configured successfully");
    Ok(())
}

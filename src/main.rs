use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use tokio;
use dirs;
use serde_json;
use std::fs;
use inquire::Select;
mod system_info;

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
    // Get API key from config
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;
    let config_file = config_dir.join("tt").join("config.json");
    
    let config_content = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_content)?;
    let api_key = config.get("openai_api_key")
        .and_then(|key| key.as_str())
        .ok_or_else(|| anyhow::anyhow!("OpenAI API key not configured"))?;
    
    // Create client with API key
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    
    // Define the response schema
    #[derive(serde::Deserialize)]
    struct CommandResponse {
        description: String,
        command: String,
    }
    
    // Get OS and shell information
    let (os_name, os_version) = system_info::get_os_info();
    let shell = system_info::get_shell_info();
    
    // Prepare request body with schema guidance
    let body = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "system",
                "content": format!("You are a helpful command assistant.\n\nCurrent User:\nOS : {}\nOS Version : {}\nShell : {}\nAlways respond in JSON format with the following structure:\n{{\n    \"description\": \"Description about the command and user's input response\",\n    \"command\": \"The actual command to execute\"\n}}\nEnsure the response is valid JSON and contains both fields.",
                os_name, os_version, shell)
            },
            {
                "role": "user",
                "content": query
            }
        ]
    });
    
    // Make request
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
    }
    
    let response_json: serde_json::Value = response.json().await?;
    let answer = response_json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .unwrap_or("No response received");
    
    // Parse JSON response
    let json_response = serde_json::from_str::<CommandResponse>(answer)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}\nRaw response: {}", e, answer))?;
    
    // Format and display the response
    println!("\n{}", ansi_term::Colour::Green.bold().paint("Summary:"));
    println!("  {}", json_response.description);
    println!("\n{}", ansi_term::Colour::Yellow.bold().paint("Command:"));
    println!("  {}", json_response.command);
    println!();
    
    // Show options menu
    let options = vec![
        "Execute command",
        // "Request edit suggestion",
        "Exit"
    ];
    
    let selected = Select::new("Choose an option:", options)
        .with_help_message("Use arrow keys to navigate")
        .prompt()?;
    
    if selected == "Execute command" {
        println!("Executing command...");
        // TODO: Implement command execution
    } else if selected == "Request edit suggestion" {
        // println!("Requesting edit suggestion...");
        // TODO: Implement edit suggestion request
    } else if selected == "Exit" {
        println!("Exiting...");
        return Ok(());
    }
    
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
    let (os_name, os_version) = system_info::get_os_info();
    let shell = system_info::get_shell_info();

    println!("\nOS: {} {}", os_name, os_version);
    println!("Shell: {}", shell);
    println!("OpenAI API Key Configured: {}", is_key_configured("openai_api_key"));
    println!();
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

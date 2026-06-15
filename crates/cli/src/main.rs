use clap::{Parser, Subcommand};
use dockyard_config::StackConfig;
use dockyard_core::domain::Container;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dockyard", author, version, about = "Dockyard Container Orchestrator CLI")]
struct Cli {
    #[arg(short, long, default_value = "http://localhost:8080", env = "DOCKYARD_API_URL")]
    api_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all containers running on the cluster nodes
    Ps,
    
    /// Start a container on the runtime
    Start {
        /// Container name or ID
        container: String,
        /// Optional image (defaults to nginx:latest if not already created)
        #[arg(short, long)]
        image: Option<String>,
    },
    
    /// Stop a container
    Stop {
        /// Container name or ID
        container: String,
    },
    
    /// Restart a container
    Restart {
        /// Container name or ID
        container: String,
    },
    
    /// View container logs
    Logs {
        /// Container name or ID
        container: String,
        /// Number of lines to show from the end of the logs
        #[arg(short, long)]
        tail: Option<usize>,
    },

    /// Apply a YAML deployment stack definition (e.g. stack.yml)
    Apply {
        /// Path to stack.yml file
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Ps => {
            let url = format!("{}/containers", cli.api_url);
            let res = client.get(&url).send().await?;
            if res.status().is_success() {
                let containers: Vec<Container> = res.json().await?;
                if containers.is_empty() {
                    println!("No containers running.");
                } else {
                    println!(
                        "{:<25} {:<25} {:<35} {:<15} {:<25}",
                        "CONTAINER ID", "NAME", "IMAGE", "STATE", "CREATED AT"
                    );
                    println!("{}", "-".repeat(125));
                    for c in containers {
                        let id = if c.id.len() > 12 { &c.id[0..12] } else { &c.id };
                        println!(
                            "{:<25} {:<25} {:<35} {:<15} {:<25}",
                            id,
                            c.name,
                            c.image,
                            c.state.to_string(),
                            c.created_at.to_rfc3339()
                        );
                    }
                }
            } else {
                eprintln!("Failed to get containers: {}", res.text().await?);
            }
        }
        
        Commands::Start { container, image } => {
            let url = format!("{}/start", cli.api_url);
            let payload = serde_json::json!({
                "id": container,
                "image": image,
            });
            let res = client.post(&url).json(&payload).send().await?;
            if res.status().is_success() {
                println!("Successfully sent start command for container '{}'.", container);
            } else {
                eprintln!("Failed to start container '{}': {}", container, res.text().await?);
            }
        }
        
        Commands::Stop { container } => {
            let url = format!("{}/stop", cli.api_url);
            let payload = serde_json::json!({
                "id": container,
            });
            let res = client.post(&url).json(&payload).send().await?;
            if res.status().is_success() {
                println!("Successfully sent stop command for container '{}'.", container);
            } else {
                eprintln!("Failed to stop container '{}': {}", container, res.text().await?);
            }
        }
        
        Commands::Restart { container } => {
            let url = format!("{}/restart", cli.api_url);
            let payload = serde_json::json!({
                "id": container,
            });
            let res = client.post(&url).json(&payload).send().await?;
            if res.status().is_success() {
                println!("Successfully sent restart command for container '{}'.", container);
            } else {
                eprintln!("Failed to restart container '{}': {}", container, res.text().await?);
            }
        }
        
        Commands::Logs { container, tail } => {
            let url = format!("{}/logs", cli.api_url);
            let mut query = vec![("id", container.clone())];
            let tail_str = tail.map(|t| t.to_string());
            if let Some(ref t) = tail_str {
                query.push(("tail", t.clone()));
            }
            
            let res = client.get(&url).query(&query).send().await?;
            if res.status().is_success() {
                let lines: Vec<String> = res.json().await?;
                for line in lines {
                    println!("{}", line);
                }
            } else {
                eprintln!("Failed to view logs for container '{}': {}", container, res.text().await?);
            }
        }
        
        Commands::Apply { file } => {
            if !file.exists() {
                eprintln!("Error: configuration file '{:?}' not found", file);
                std::process::exit(1);
            }
            let content = tokio::fs::read_to_string(&file).await?;
            let config = StackConfig::parse_yaml(&content)?;
            let url = format!("{}/deploy", cli.api_url);
            
            println!("Applying stack deployment configuration: {:?}", file);
            for (name, service_cfg) in config.services {
                let payload = serde_json::json!({
                    "name": name,
                    "image": service_cfg.image,
                    "replicas": service_cfg.replicas,
                });
                let res = client.post(&url).json(&payload).send().await?;
                if res.status().is_success() {
                    println!("  - Service '{}' defined with image '{}' and replicas = {}", name, service_cfg.image, service_cfg.replicas.unwrap_or(1));
                } else {
                    eprintln!("  - Failed to deploy service '{}': {}", name, res.text().await?);
                }
            }
            println!("Stack successfully applied.");
        }
    }

    Ok(())
}

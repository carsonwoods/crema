#![deny(unsafe_code)]
use std::env;
use std::error::Error;
use std::time::Duration;

use clap::{Arg, Command};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let access_token = env::var("TERMINAL_BEARER_TOKEN").unwrap_or_else(|_| {
        eprintln!("ERROR: TERMINAL_BEARER_TOKEN environment variable is not set.");
        std::process::exit(1);
    });

    // Define the main command, crema, and its subcommands
    let matches = Command::new("crema")
        .version("1.0")
        .author("Carson Woods <you@example.com>")
        .about("CLI for interacting with your Terminal.shop account")
        .subcommand_required(true) // Enforce that a subcommand is required
        .subcommand(
            Command::new("profile")
                .about("Manage user profiles")
                .subcommand_required(true) // Enforce that a subcommand is required
                .subcommand(
                    Command::new("get").about("Get profile metadata").arg(
                        Arg::new("get")
                            .long("get")
                            .help("Get profile metadata")
                            .action(clap::ArgAction::SetTrue), // Set to true when flag is provided
                    ),
                ),
        )
        .subcommand(
            Command::new("product")
                .about("Retrieve product information")
                .subcommand_required(true) // Enforce that a subcommand is required
                .subcommand(
                    Command::new("list")
                        .about("List all products and corresponding product ID")
                        .arg(
                            Arg::new("list")
                                .long("list")
                                .help("Retrieves product list")
                                .action(clap::ArgAction::SetTrue), // Set to true when flag is provided
                        ),
                ),
        )
        .get_matches();

    // Match against the subcommands
    match matches.subcommand() {
        Some(("profile", profile_matches)) => match profile_matches.subcommand() {
            Some(("get", _)) => {
                let client = Client::new();
                let url = "https://api.terminal.shop/profile";

                let response = client
                    .get(url)
                    .bearer_auth(access_token)
                    .timeout(Duration::from_secs(10)) // Avoid hanging
                    .send()
                    .await?;

                if response.status().is_success() {
                    let body = response.text().await?;
                    println!("Response: {}", body);
                } else {
                    eprintln!("Error: {}", response.status());
                }
            }
            _ => println!("Unknown profile command"),
        },
        Some(("product", config_matches)) => match config_matches.subcommand() {
            Some(("list", _)) => crema::product::list(access_token.as_str()).await,
            Some(("reset", _)) => {
                println!("Resetting configuration to defaults...");
            }
            _ => println!("Unknown config command"),
        },
        _ => println!("Unknown command"),
    }

    Ok(())
}

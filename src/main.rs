pub mod commands;

use clap::{Parser, Subcommand};
use commands::config::InitConfig;
use commands::config::init_config;
use commands::init_challenge::init_challenge;
use commands::init_challenge::InitChallenge;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "aoc-initializer-cli - initializes code challenges for AOC",
    long_about = "aoc-initializer-cli - This CLI was created to allow you to quickly initialize code challenges for Advent Of Code."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the config.json
    Config(InitConfig),
    // Initializes a days coding challenge
    Init(InitChallenge),
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let some = match &cli.command {
        Some(Commands::Config(conf)) => init_config(&conf.aoc_key, &conf.year),
        Some(Commands::Init(challenge)) => init_challenge(&challenge.day).await,
        None => panic!("Unknown command"),
    };

    if some.is_err() {
       return Err(some.unwrap_err().to_string());
    }

    Ok(())
}

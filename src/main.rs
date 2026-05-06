mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Watch => {
            commands::watch().await?;
        }

        Commands::Blocks {
            from,
            to,
            concurrency,
        } => {
            commands::blocks(from, to, concurrency).await?;
        }

        Commands::Tx { hash } => {
            commands::tx(hash).await?;
        }
    }

    Ok(())
}

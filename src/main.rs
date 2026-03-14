mod cli;
mod client;
mod commands;
mod config;
mod models;
mod output;

use anyhow::Result;
use clap::Parser;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli).await
}

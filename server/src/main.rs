mod cli;
mod error;
mod init;
mod model;
mod server;

use crate::{cli::Args, init::init, server::start_server};
use anyhow::Result;
use clap::Parser as _;
use std::process::ExitCode;
use tracing::error;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = run().await {
        error!("{}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run() -> Result<()> {
    init()?;
    let args = Args::parse();
    start_server(args.port).await?;
    Ok(())
}

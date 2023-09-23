use clap::Parser;
use std::process;
use tracing::Level;

use typewriter::args::Args;
use typewriter::error::Result;
use typewriter::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Initialize the logger.
    logger::init(args.verbose.then_some(Level::DEBUG))?;
    tracing::info!("starting");

    // Run the typewriter.
    match typewriter::run().await {
        Ok(_) => process::exit(0),
        Err(e) => {
            tracing::error!("error occurred: {e}");
            process::exit(1)
        }
    }
}

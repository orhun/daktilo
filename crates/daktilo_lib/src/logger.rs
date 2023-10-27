use crate::error::Result;
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes the logger with the given default log level.
pub fn init(default_level: Option<Level>) -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .from_env_lossy()
                .add_directive("none".parse()?)
                .add_directive(
                    format!(
                        "{}={}",
                        env!("CARGO_PKG_NAME"),
                        default_level.unwrap_or(Level::INFO)
                    )
                    .parse()?,
                ),
        )
        .init();
    Ok(())
}

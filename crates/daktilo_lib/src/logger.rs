use crate::error::Result;
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes the logger with the given default log level.
///
/// Provide your crate names to enable logging for them.
pub fn init(default_level: Option<Level>, mut crates: Vec<String>) -> Result<()> {
    crates.push(env!("CARGO_PKG_NAME").to_string());
    let mut env_filter = EnvFilter::builder()
        .from_env_lossy()
        .add_directive("none".parse()?);
    for crate_name in crates {
        env_filter = env_filter.add_directive(
            format!("{}={}", crate_name, default_level.unwrap_or(Level::INFO)).parse()?,
        );
    }
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    Ok(())
}

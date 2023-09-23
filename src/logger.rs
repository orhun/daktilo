use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes the logger with the given default log level.
pub fn init(default_level: Option<Level>) {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(default_level.unwrap_or(Level::INFO).into())
                .from_env_lossy(),
        )
        .init();
}

use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),

    /// Error that may occur during audio streams.
    #[error("Stream error: `{0}`")]
    Stream(#[from] rodio::StreamError),

    /// Error that may occur while decoding data.
    #[error("Decode error: `{0}`")]
    Decode(#[from] rodio::decoder::DecoderError),

    /// Error that may occur during playing audio.
    #[error("Play error: `{0}`")]
    Play(#[from] rodio::PlayError),

    /// Error that may occur when an embedded asset is not found.
    #[error("Asset not found: `{0}`")]
    AssetNotFound(String),

    /// Error that may occur while parsing a filtering directive.
    #[error("Log directive parse error: `{0}`")]
    DirectiveParse(#[from] tracing_subscriber::filter::ParseError),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::new(ErrorKind::Other, message));
        assert_eq!(format!("IO error: `{message}`"), error.to_string());
        assert_eq!(
            format!("\"IO error: `{message}`\""),
            format!("{:?}", error.to_string())
        );
    }
}

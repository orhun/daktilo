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

    /// Error that may occur during playing audio.
    #[error("Play error: `{0}`")]
    Play(#[from] rodio::PlayError),
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

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

    /// Error that may occur while extracting the embedded content.
    #[error("Embedded error: `{0}`")]
    Embedded(String),

    /// Error that may occur while parsing a filtering directive.
    #[error("Log directive parse error: `{0}`")]
    DirectiveParse(#[from] tracing_subscriber::filter::ParseError),

    /// Error that may occur while parsing TOML.
    #[error("TOML parse error: `{0}`")]
    TomlParse(#[from] toml::de::Error),

    /// Error that may occur when attempting to interpret a sequence of u8 as a
    /// string.
    #[error("UTF-8 error: `{0}`")]
    Utf8(#[from] std::str::Utf8Error),

    /// Error that may occur when a preset is not found.
    #[error("Preset not found: `{0}`")]
    PresetNotFound(String),

    /// Error that may occur when no audio files are given.
    #[error("No audio files to play")]
    NoAudioFiles,

    /// Error that may occur when parsing regexes.
    #[error("Regex: `{0}`")]
    Regex(#[from] regex::Error),
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

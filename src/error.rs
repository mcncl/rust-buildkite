use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// The HTTP request responded with some failure (DNS, TLS, timout...)
    Http(String),
    /// Buildkite returned a non-2xx status
    Api { status: u16, message: String },
    /// We couldn't parse the JSON returned by Buildkite
    Decode(String),
    /// An authenticated endpoint was called on a client created without a token
    MissingToken,
}

/// Alias so that functions can write `Result<Pipeline>` instead of
/// `Result<Pipeline, Error>`
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(msg) => write!(f, "http error: {msg}"),
            Error::Api { status, message } => {
                write!(f, "api error {status}: {message}")
            }
            Error::Decode(msg) => write!(f, "decode error: {msg}"),
            Error::MissingToken => {
                write!(f, "missing token: this endpoint requires authentication")
            }
        }
    }
}

impl std::error::Error for Error {}

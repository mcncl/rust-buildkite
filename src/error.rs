#[derive(Debug)]
pub enum Error {
    /// The HTTP request responded with some failure (DNS, TLS, timout...)
    HTTP(String),
    /// Buildkite returned a non-2xx status
    API { status: u16, message: String },
    /// We couldn't parse the JSON returned by Buildkite
    Decode(String),
}

/// Alias so that functions can write `Result<Pipeline>` instead of
/// `Result<Pipeline, Error>`
pub type Result<T> = std::result::Result<T, Error>;

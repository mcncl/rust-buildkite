use serde::{Deserialize, Serialize};

/// Response returned by the unauthenticated health-check endpoint
/// (`GET https://api.buildkite.com/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    pub message: String,
    pub timestamp: u64,
}

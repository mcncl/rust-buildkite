use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Build {
    pub number: u64,
    pub state: BuildState,
    pub message: Option<String>,
}

// BuildState represents the states which a Buildkite build can be in
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildState {
    Running,
    Passed,
    Failed,
    Canceled,
    Scheduled,
}

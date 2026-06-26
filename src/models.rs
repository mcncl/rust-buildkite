#[derive(Debug, Clone)]
pub struct Pipeline {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub repository: String,
}

#[derive(Debug, Clone)]
pub struct Build {
    pub number: u64,
    pub state: BuildState,
    pub message: Option<String>,
}

// BuildState represents the states which a Buildkite build can be in
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildState {
    Running,
    Passed,
    Failed,
    Canceled,
    Scheduled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_has_state() {
        let _b = Build {
            number: 1,
            state: BuildState::Passed,
            message: None,
        };
    }
}

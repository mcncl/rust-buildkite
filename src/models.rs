use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

    #[test]
    fn parses_pipeline_json() {
        let json = r#"{
            "id": "849411f9-9e6d-4739-a0d8-e247088e9b52",
            "name": "My Pipeline",
            "slug": "my-pipeline",
            "repository": "git@github.com:acme-inc/my-pipeline.git"
        }"#;

        let pipeline: Pipeline = serde_json::from_str(json).unwrap();
        assert_eq!(pipeline.slug, "my-pipeline");

        let back = serde_json::to_string(&pipeline).unwrap();
        let reparsed: Pipeline = serde_json::from_str(&back).unwrap();
        assert_eq!(reparsed.name, "My Pipeline")
    }
}

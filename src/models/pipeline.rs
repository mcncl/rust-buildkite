use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreatePipelineInput {
    pub name: String,
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

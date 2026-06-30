use crate::client::Client;
use crate::error::Result;
use crate::models::Pipeline;
use crate::models::pipeline::CreatePipelineInput;

pub struct Pipelines<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Pipelines<'a> {
    pub async fn get(&self, org: &str, slug: &str) -> Result<Pipeline> {
        let path = format!("v2/organizations/{org}/pipelines/{slug}");
        self.client.get_json(&path, true).await
    }

    pub async fn list(&self, org: &str) -> Result<Vec<Pipeline>> {
        let path = format!("v2/organizations/{org}/pipelines");
        self.client.get_json(&path, true).await
    }

    pub async fn create(&self, org: &str, input: CreatePipelineInput) -> Result<Pipeline> {
        let path = format!("v2/organizations/{org}/pipelines");
        self.client.post_json(&path, true, input).await
    }
}

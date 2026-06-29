use crate::client::Client;
use crate::error::Result;
use crate::models::Pipeline;

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
}

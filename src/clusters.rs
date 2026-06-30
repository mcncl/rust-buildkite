use crate::client::Client;
use crate::error::Result;
use crate::models::Cluster;

pub struct Clusters<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Clusters<'a> {
    pub async fn list(&self, org: &str) -> Result<Vec<Cluster>> {
        let path: String = format!("v2/organizations/{org}/clusters");
        self.client.get_json(&path, true).await
    }

    pub async fn get(&self, org: &str, id: &str) -> Result<Cluster> {
        let path: String = format!("v2/organizations/{org}/clusters/{id}");
        self.client.get_json(&path, true).await
    }
}

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::models::Ping;
use crate::pipelines::Pipelines;

const DEFAULT_BASE_URL: &str = "https://api.buildkite.com/";

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl Client {
    /// Create a client, optionally authenticated.
    ///
    /// Pass `Some(token)` for endpoints that require authentication, or
    /// `None` for unauthenticated use (e.g. the [`ping`](Self::ping)
    /// health check).
    pub fn new(token: Option<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            token,
        }
    }

    /// Return the configured token, or `Error::MissingToken` if the client
    /// was created without one. Use this for endpoints that require auth.
    fn token(&self) -> Result<&str> {
        self.token.as_deref().ok_or(Error::MissingToken)
    }

    /// Health check against the API root. Requires no authentication.
    pub async fn ping(&self) -> Result<Ping> {
        let resp = self
            .http
            .get(&self.base_url)
            .send()
            .await
            .map_err(|e| Error::Http(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        resp.json::<Ping>()
            .await
            .map_err(|e| Error::Decode(e.to_string()))
    }

    pub fn pipelines(&self) -> Pipelines<'_> {
        Pipelines { client: self }
    }

    /// Shared GET: build URL, optionally auth, check status, decode JSON.
    /// `pub(crate)` = visible to sibling modules, invisible outside the crate.
    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        authenticated: bool,
    ) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        let mut req = self.http.get(&url);
        if authenticated {
            req = req.bearer_auth(self.token()?);
        }
        let resp = req.send().await.map_err(|e| Error::Http(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        resp.json::<T>()
            .await
            .map_err(|e| Error::Decode(e.to_string()))
    }
}

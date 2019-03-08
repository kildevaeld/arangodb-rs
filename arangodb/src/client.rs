use super::error::{ErrorKind, Result};
use arangodb_http::api_support::auth::Authorization;
use arangodb_http::api_support::reqwest::r#async::Client as HttpClient;
use std::sync::Arc;
pub struct ClientBuilder {
    endpoint: Option<String>,
    auth: Option<Authorization>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            endpoint: None,
            auth: None,
        }
    }

    pub fn endpoint<T: AsRef<str>>(mut self, endpoint: T) -> Self {
        self.endpoint = Some(endpoint.as_ref().to_owned());
        self
    }

    pub fn auth(mut self, auth: Authorization) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> Result<Arc<Client>> {
        if self.auth.is_none() {
            bail!(ErrorKind::Authorization);
        }
        if self.endpoint.is_none() {
            bail!(ErrorKind::Endpoint);
        }
        let client = HttpClient::new();

        Ok(Arc::new(Client {
            client,
            auth: self.auth.unwrap(),
            endpoint: self.endpoint.unwrap(),
        }))
    }
}

pub struct Client {
    pub(crate) client: HttpClient,
    pub(crate) endpoint: String,
    pub(crate) auth: Authorization,
}

impl Client {
    pub fn build() -> ClientBuilder {
        ClientBuilder::new()
    }
}

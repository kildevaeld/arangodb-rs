use super::client::Client;
use super::error::Error;
use arangodb_http::api_support::futures::{self, Future};

pub trait Query: Send + Sync {
    type Item;
    fn query(self, client: &Client) -> Box<dyn Future<Item = Self::Item, Error = Error>>;
}

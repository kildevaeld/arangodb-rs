use super::client::Client;
use super::error::Error;
use super::types::Query;
use arangodb_http::api_support::futures::{self, Future};
use arangodb_http::{DatabaseService, PostApiDatabaseBody, Response, User};
use std::sync::Arc;

pub struct ListDatabases;

impl ListDatabases {
    pub fn new() -> ListDatabases {
        ListDatabases {}
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, Vec<String>), Error = Error> {
        DatabaseService::with_client(&client.client, &client.endpoint)
            .get__api_database(client.auth.clone())
            .from_err::<Error>()
            .map(|m| (client, m.result))
    }
}

pub struct CreateDatabase {
    body: PostApiDatabaseBody,
}

impl CreateDatabase {
    pub fn new<T: AsRef<str>>(name: T) -> CreateDatabase {
        CreateDatabase {
            body: PostApiDatabaseBody {
                name: name.as_ref().to_owned(),
                users: vec![],
            },
        }
    }

    pub fn user(mut self, user: User) -> Self {
        self.body.users.push(user);
        self
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, bool), Error = Error> {
        DatabaseService::with_client(&client.client, &client.endpoint)
            .post__api_database(client.auth.clone(), self.body)
            .from_err::<Error>()
            .map(|m| (client, m.result))
    }
}

// impl Query for ListDatabases {
//     type Item = Vec<String>;
//     fn query(self, client: &Client) -> Box<Future<Item = Self::Item, Error = Error>> {
//         Box::new(self.run(client))
//     }
// }

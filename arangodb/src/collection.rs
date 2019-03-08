use super::client::Client;
use super::error::Error;
use super::types::Query;
use arangodb_http::api_support::futures::{self, Future};
use arangodb_http::{
    Collection, CollectionProperties, CollectionService, GetApiCollectionQuery,
    PostApiCollectionQuery, PostApiCollectionResult, Response, User,
};
use std::sync::Arc;

pub struct ListCollections;

impl ListCollections {
    pub fn new() -> ListCollections {
        ListCollections {}
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, Vec<Collection>), Error = Error> {
        CollectionService::with_client(&client.client, &client.endpoint)
            .get__api_collection(client.auth.clone(), GetApiCollectionQuery::default())
            .from_err::<Error>()
            .map(|m| (client, m.result))
    }
}

pub struct CreateCollection {
    query: PostApiCollectionQuery,
    body: CollectionProperties,
}

impl CreateCollection {
    pub fn new<T: AsRef<str>>(m: T) -> CreateCollection {
        let mut body = CollectionProperties {
            journalSize: Some(1048576),
            replicationFactor: Some(1),
            keyOptions: None,
            name: Some(m.as_ref().to_owned()),
            waitForSync: Some(false),
            doCompact: Some(true),
            isVolatile: Some(false),
            numberOfShards: Some(1),
            isSystem: Some(false),
            type_: Some(2),
            indexBuckets: Some(16),
            distributeShardsLike: Some(String::from("")),
        };

        let query = PostApiCollectionQuery {
            waitForSyncReplication: Some(1),
            enforceReplicationFactor: Some(1),
        };

        CreateCollection { query, body }
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, PostApiCollectionResult), Error = Error> {
        CollectionService::with_client(&client.client, &client.endpoint)
            .post__api_collection(client.auth.clone(), self.query, self.body)
            .from_err::<Error>()
            .map(|m| (client, m))
    }
}

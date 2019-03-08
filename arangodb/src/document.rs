use super::client::Client;
use super::error::Error;
use super::types::Query;
use arangodb_http::api_support::futures::{self, Future};
use arangodb_http::{
    Collection, CollectionProperties, CollectionService, DocumentService, GetApiCollectionQuery,
    PostApiCollectionQuery, PostApiCollectionResult, PostApiDocumentCollectionQuery, Response,
    User,
};
use std::sync::Arc;

struct CreateDocument {
    name: String,
    query: PostApiDocumentCollectionQuery,
}

impl CreateDocument {
    pub fn new<T: AsRef<str>>(collection_name: T) -> Docum {
        CreateDocument {
            name: collection_name.as_ref().to_owned(),
            query: PostApiDocumentCollectionQuery {
                waitForSync: Some(false),
                returnNew: Some(false),
                silent: Some(false),
            },
        }
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, PostApiCollectionResult), Error = Error> {
        DocumentService::with_client(&client.client, &client.endpoint)
            .post__api_document_collection(&self.name, client.auth.clone(), self.query)
            .from_err::<Error>()
            .map(|m| (client, m))
    }
}

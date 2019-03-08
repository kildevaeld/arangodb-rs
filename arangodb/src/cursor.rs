use super::client::Client;
use super::error::{Error, ErrorKind};
use super::types::Query;
use arangodb_http::api_support::futures::{self, Async, Future, Poll};
use arangodb_http::{
    api_support, Collection, CollectionProperties, CollectionService, Cursor as CursorResponse,
    CursorService, DocumentService, GetApiCollectionQuery, PostApiCollectionQuery,
    PostApiCollectionResult, PostApiCursorBody, PostApiCursorBodyOptions,
    PostApiDocumentCollectionQuery, Response, User,
};
use std::sync::Arc;

pub struct CreateCursor {
    query: PostApiCursorBody,
}

impl CreateCursor {
    pub fn new<T: AsRef<str>>(query: T) -> CreateCursor {
        CreateCursor {
            query: PostApiCursorBody {
                count: Some(false),
                batchSize: None,
                cache: None,
                memoryLimit: None,
                ttl: None,
                query: query.as_ref().to_owned(),
                bindVars: None,
                options: PostApiCursorBodyOptions::default(),
            },
        }
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, CursorResponse), Error = Error> {
        CursorService::with_client(&client.client, &client.endpoint)
            .post__api_cursor(client.auth.clone(), self.query)
            .from_err::<Error>()
            .map(|m| (client, m))
    }
}

pub struct NextCursor {
    cursor_id: String,
}

impl NextCursor {
    pub fn new<T: AsRef<str>>(cursor_id: T) -> NextCursor {
        NextCursor {
            cursor_id: cursor_id.as_ref().to_owned(),
        }
    }

    pub fn run(
        self,
        client: Arc<Client>,
    ) -> impl Future<Item = (Arc<Client>, CursorResponse), Error = Error> {
        CursorService::with_client(&client.client, &client.endpoint)
            .put__api_cursor_cursor_identifier(&self.cursor_id, client.auth.clone())
            .from_err::<Error>()
            .map(|m| (client, m))
    }
}

extern crate arangodb_http;
#[macro_use]
extern crate error_chain;

mod client;
mod collection;
mod cursor;
mod database;
pub mod error;
mod types;

pub use self::client::Client;
pub use self::collection::*;
pub use self::cursor::*;
pub use self::database::*;
pub use self::types::Query;
pub use arangodb_http::api_support::auth::Authorization;
pub use arangodb_http::api_support::futures::{future::lazy, Future};
pub use arangodb_http::api_support::tokio as rt;

pub mod models {
    pub use arangodb_http::{Cursor, Response, User};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

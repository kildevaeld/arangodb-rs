use arangodb_http::api_support::error;
error_chain!{
    links {
        Api(error::Error, error::ErrorKind);
    }
    errors {
        Endpoint {
            description("invalid endpoint")
            display("invalid endpoint")
        }
        Authorization {
            description("AuthorizationError")
            display("Not authoriz")
        }
    }
}

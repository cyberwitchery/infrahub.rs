//! error types
//!
//! structured errors for config, http, json, and graphql responses.

use crate::graphql::GraphQlError;
use std::fmt;

/// library result type
pub type Result<T> = std::result::Result<T, Error>;

/// error type for client and codegen helpers
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config error: {0}")]
    Config(String),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("url error: {0}")]
    Url(#[from] url::ParseError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("graphql error: {message}")]
    GraphQl {
        /// http status if available
        status: Option<u16>,
        /// graphql error list
        errors: Vec<GraphQlError>,
        /// raw response body
        body: String,
        /// top-level message
        message: String,
    },
}

impl Error {
    /// true if the error looks like an auth failure
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::GraphQl { status: Some(401 | 403), .. })
            || matches!(self, Error::Http(err) if err.status() == Some(reqwest::StatusCode::UNAUTHORIZED))
    }
}

impl fmt::Display for GraphQlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_auth_error() {
        let err = Error::GraphQl {
            status: Some(401),
            errors: vec![],
            body: String::new(),
            message: "unauthorized".to_string(),
        };
        assert!(err.is_auth_error());

        let err = Error::GraphQl {
            status: Some(403),
            errors: vec![],
            body: String::new(),
            message: "forbidden".to_string(),
        };
        assert!(err.is_auth_error());

        let err = Error::GraphQl {
            status: Some(500),
            errors: vec![],
            body: String::new(),
            message: "server error".to_string(),
        };
        assert!(!err.is_auth_error());
    }
}

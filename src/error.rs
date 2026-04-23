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
        matches!(
            self,
            Error::GraphQl {
                status: Some(401 | 403),
                ..
            }
        ) || matches!(self, Error::Http(err) if err.status() == Some(reqwest::StatusCode::UNAUTHORIZED))
    }

    /// true if the error is transient and the request may succeed on retry
    ///
    /// auth errors, config errors, and parse errors are permanent.
    /// server errors (5xx), rate limits (429), and network errors are retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Config(_) | Error::Url(_) | Error::Json(_) => false,
            Error::Http(err) => {
                if err.is_timeout() || err.is_connect() {
                    return true;
                }
                match err.status() {
                    Some(status) => {
                        status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS
                    }
                    // no status usually means a network-level failure
                    None => true,
                }
            }
            Error::GraphQl { status, .. } => {
                matches!(status, Some(429 | 500 | 502 | 503 | 504))
            }
        }
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

    #[test]
    fn test_non_graphql_errors_not_auth() {
        let config_err = Error::Config("bad".into());
        assert!(!config_err.is_auth_error());

        let url_err: Error = url::Url::parse(":::").unwrap_err().into();
        assert!(!url_err.is_auth_error());

        let json_err: Error = serde_json::from_str::<serde_json::Value>("!!!")
            .unwrap_err()
            .into();
        assert!(!json_err.is_auth_error());
    }

    #[test]
    fn test_is_retryable_server_errors() {
        for status in [429, 500, 502, 503, 504] {
            let err = Error::GraphQl {
                status: Some(status),
                errors: vec![],
                body: String::new(),
                message: "server error".to_string(),
            };
            assert!(err.is_retryable(), "status {status} should be retryable");
        }
    }

    #[test]
    fn test_is_not_retryable_client_errors() {
        for status in [400, 401, 403, 404, 422] {
            let err = Error::GraphQl {
                status: Some(status),
                errors: vec![],
                body: String::new(),
                message: "client error".to_string(),
            };
            assert!(
                !err.is_retryable(),
                "status {status} should not be retryable"
            );
        }
    }

    #[test]
    fn test_is_not_retryable_config_url_json() {
        let config_err = Error::Config("bad".into());
        assert!(!config_err.is_retryable());

        let url_err: Error = url::Url::parse(":::").unwrap_err().into();
        assert!(!url_err.is_retryable());

        let json_err: Error = serde_json::from_str::<serde_json::Value>("!!!")
            .unwrap_err()
            .into();
        assert!(!json_err.is_retryable());
    }
}

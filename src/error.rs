//! error types
//!
//! structured errors for config, http, json, pagination, and graphql responses.

use crate::graphql::GraphQlError;
use std::fmt;

/// graphql response status codes considered transient and worth retrying
const RETRYABLE_STATUSES: &[u16] = &[429, 500, 502, 503, 504];

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

    #[error("pagination error: server repeated cursor after {pages} page(s)")]
    PaginationStalled {
        /// pages handed to the caller before the repeated cursor arrived
        pages: usize,
    },

    #[error("pagination error: exceeded max_pages limit of {max_pages}")]
    PaginationLimit {
        /// configured page ceiling
        max_pages: usize,
    },

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
        )
    }

    /// true if the error is transient and the request may succeed on retry
    ///
    /// config, url, and parse errors are permanent. http errors retry unless
    /// the request could not be built or the redirect policy rejected it;
    /// graphql errors retry only on 429, 500, 502, 503, and 504.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Config(_)
            | Error::Url(_)
            | Error::Json(_)
            | Error::PaginationStalled { .. }
            | Error::PaginationLimit { .. } => false,
            Error::Http(err) => !(err.is_builder() || err.is_redirect()),
            Error::GraphQl { status, .. } => {
                status.is_some_and(|s| RETRYABLE_STATUSES.contains(&s))
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
    fn test_is_retryable_and_is_auth_error() {
        fn graphql(status: u16) -> Error {
            Error::GraphQl {
                status: Some(status),
                errors: vec![],
                body: String::new(),
                message: String::new(),
            }
        }

        let builder: Error = reqwest::multipart::Part::bytes(vec![])
            .mime_str("text")
            .unwrap_err()
            .into();

        // (label, error, is_retryable, is_auth_error)
        let cases: Vec<(&str, Error, bool, bool)> = vec![
            ("graphql 401", graphql(401), false, true),
            ("graphql 403", graphql(403), false, true),
            ("graphql 429", graphql(429), true, false),
            ("graphql 500", graphql(500), true, false),
            ("graphql 502", graphql(502), true, false),
            ("graphql 503", graphql(503), true, false),
            ("graphql 504", graphql(504), true, false),
            ("graphql 400", graphql(400), false, false),
            ("graphql 404", graphql(404), false, false),
            ("graphql 422", graphql(422), false, false),
            ("config", Error::Config("bad".into()), false, false),
            (
                "url",
                url::Url::parse(":::").unwrap_err().into(),
                false,
                false,
            ),
            (
                "json",
                serde_json::from_str::<serde_json::Value>("!!!")
                    .unwrap_err()
                    .into(),
                false,
                false,
            ),
            (
                "pagination stalled",
                Error::PaginationStalled { pages: 3 },
                false,
                false,
            ),
            (
                "pagination limit",
                Error::PaginationLimit { max_pages: 10 },
                false,
                false,
            ),
            ("http builder", builder, false, false),
        ];

        for (label, err, retryable, auth) in cases {
            assert_eq!(err.is_retryable(), retryable, "{label}: is_retryable");
            assert_eq!(err.is_auth_error(), auth, "{label}: is_auth_error");
        }
    }

    #[test]
    fn test_pagination_error_messages() {
        assert_eq!(
            Error::PaginationStalled { pages: 4 }.to_string(),
            "pagination error: server repeated cursor after 4 page(s)"
        );
        assert_eq!(
            Error::PaginationLimit { max_pages: 7 }.to_string(),
            "pagination error: exceeded max_pages limit of 7"
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_connect_error_is_retryable() {
        let addr = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap();
        let err: Error = reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap()
            .get(format!("http://{addr}"))
            .send()
            .await
            .unwrap_err()
            .into();
        assert!(err.is_retryable());
    }
}

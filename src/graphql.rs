//! graphql types
//!
//! wrappers for graphql responses and errors.

use serde::{Deserialize, Serialize};

/// graphql response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlResponse<T> {
    /// response data or null if errors
    pub data: Option<T>,
    /// graphql errors array
    #[serde(default)]
    pub errors: Vec<GraphQlError>,
}

impl<T> GraphQlResponse<T> {
    /// true if the response contains graphql errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// graphql error entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlError {
    /// error message
    pub message: String,
    /// error locations in the query
    #[serde(default)]
    pub locations: Vec<GraphQlLocation>,
    /// response path
    #[serde(default)]
    pub path: Vec<serde_json::Value>,
    /// optional extensions payload
    #[serde(default)]
    pub extensions: Option<serde_json::Value>,
}

/// graphql error location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlLocation {
    /// line number (1-based)
    pub line: i64,
    /// column number (1-based)
    pub column: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_errors() {
        let ok: GraphQlResponse<serde_json::Value> = GraphQlResponse {
            data: Some(serde_json::json!({"ok": true})),
            errors: vec![],
        };
        assert!(!ok.has_errors());

        let err = GraphQlResponse::<serde_json::Value> {
            data: None,
            errors: vec![GraphQlError {
                message: "boom".to_string(),
                locations: vec![],
                path: vec![],
                extensions: None,
            }],
        };
        assert!(err.has_errors());
    }
}

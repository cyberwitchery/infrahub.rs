//! generated operation helper
//!
//! operation trait implemented by generated types.

use serde::de::DeserializeOwned;

/// graphql operation contract for generated types
pub trait Operation {
    /// graphql query or mutation string
    const QUERY: &'static str;
    /// response payload type
    type Response: DeserializeOwned;
}

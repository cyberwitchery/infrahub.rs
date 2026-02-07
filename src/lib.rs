//! infrahub graphql client
//!
//! this crate provides a small, typed client for the infrahub graphql api.
//! start with [`Client`] and [`ClientConfig`], then use `execute_raw` or
//! `execute` for ad-hoc queries. for generated schema clients, use the
//! `infrahub-codegen` tool to build a separate crate.
//!
//! ## quick start
//!
//! ```no_run
//! use infrahub::{Client, ClientConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
//! let response = client
//!     .execute_raw("{ InfrahubInfo { deployment_id version } }", None, None)
//!     .await?;
//! println!("{:?}", response.data);
//! # Ok(())
//! # }
//! ```
//!
//! ## generated clients
//!
//! use `infrahub-codegen` to generate a schema-specific crate, then use it
//! alongside this base client.

mod client;
mod config;
mod error;
mod graphql;
mod operation;
mod pagination;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use graphql::{GraphQlError, GraphQlLocation, GraphQlResponse};
pub use operation::Operation;
pub use pagination::{BoxExtract, BoxFetch, BoxFutureResult, DynPaginator, EdgePage, Paginator};

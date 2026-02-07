# client library

this crate provides a small, typed graphql client for infrahub.

## features

- graphql execution with raw and typed helpers
- schema fetch helper
- edges/node pagination helper
- structured errors with status and graphql details

## surface

- `Client` - graphql client with auth and branch routing
- `ClientConfig` - base url, token, timeouts, headers
- `Operation` - generated operation trait
- `Paginator` - edge/connection pagination helper

## how to use this crate

this crate is the base graphql client. you can use it directly with ad‑hoc
queries, or pair it with a generated, schema‑specific crate produced by
`infrahub-codegen`.

use cases:

- ad‑hoc graphql: use `execute_raw` / `execute` with your own query strings
- schema‑specific ergonomics: generate a crate and call its api from this client

## quick start (ad‑hoc)

```rust
use infrahub::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let response = client
    .execute_raw("{ Branch { id name } }", None, None)
    .await?;
println!("{:?}", response.data);
# Ok(())
# }
```

## install

```toml
[dependencies]
infrahub = "0.0.1"
tokio = { version = "1", features = ["full"] }
```

## config

```rust,no_run
use std::time::Duration;
use infrahub::ClientConfig;
use reqwest::header::{HeaderName, HeaderValue};

let config = ClientConfig::new("http://localhost:8000", "token")
    .with_default_branch("main")
    .with_timeout(Duration::from_secs(60))
    .with_ssl_verification(false)
    .with_header(
        HeaderName::from_static("x-client"),
        HeaderValue::from_static("infrahub-rs"),
    );
```

## typed queries

```rust,no_run
use infrahub::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let response = client
    .execute::<serde_json::Value>(
        "query($name: String!) { Branch(name__value: $name) { edges { node { id name } } } }",
        Some(serde_json::json!({ "name": "main" })),
        None,
    )
    .await?;
println!("{:?}", response.data);
# Ok(())
# }
```

## generated client

use `infrahub-codegen` to generate a schema-specific crate, then call into it
from your base client.

the generated crate provides:

- `generated()` for full surface graphql methods
- `api()` for ergonomic, topic-grouped helpers (`list`, `get_by_id`, `paginate`, plus mutation helpers when available in your schema snapshot)

## branches

branches are routed by url: `POST {base}/graphql/{branch}` and `GET {base}/schema.graphql?branch=foo`.

```rust,no_run
use infrahub::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let response = client
    .execute_raw("{ Branch { id name } }", None, Some("feature-a"))
    .await?;
println!("{:?}", response.data);
# Ok(())
# }
```

## schema fetch

```rust,no_run
use infrahub::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let schema = client.fetch_schema(None).await?;
println!("schema len: {}", schema.len());
# Ok(())
# }
```

## pagination helper

`Paginator` is generic and does not assume a pageInfo shape. pass a fetch function and an extract function.

```rust,no_run
use infrahub::{Client, ClientConfig, EdgePage, Paginator, Result};

# async fn example() -> Result<()> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;

let fetch = |cursor: Option<String>| async {
    let vars = serde_json::json!({ "after": cursor });
    client.execute_raw("query($after: String) { Branch(after: $after) { edges { node { id name } cursor } } }", Some(vars), None).await
};

let extract = |response| {
    let data = response
        .data
        .ok_or_else(|| infrahub::Error::Config("missing data".to_string()))?;
    let edges = data["Branch"]["edges"].as_array().cloned().unwrap_or_default();
    let mut nodes = Vec::new();
    let mut next = None;
    for edge in edges {
        if let Some(node) = edge.get("node") {
            nodes.push(node.clone());
        }
        next = edge.get("cursor").and_then(|c| c.as_str()).map(|s| s.to_string());
    }
    Ok(EdgePage { nodes, next_cursor: next })
};

let mut paginator = Paginator::new(fetch, extract);
let _all = paginator.collect_all().await?;
Ok(())
# }
```

## codegen

generate a full typed client from a schema snapshot:

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

then add it as a path dependency:

```toml
[dependencies]
infrahub = "0.0.1"
infrahub-generated = { path = "/tmp/infrahub-generated" }
```

usage example:

```rust,no_run
use infrahub::{Client, ClientConfig};
use infrahub_generated::ApiClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let repository_api = client.api().core().repository();

let repositories = repository_api.list(None, None).await?;
println!("repositories: {}", repositories.len());

if let Some(first) = repositories.first() {
    let fetched = repository_api.get_by_id(first.id.clone(), None).await?;
    println!("fetched: {}", fetched.is_some());
}

let mut paginator = repository_api.paginate(None, None);
let first_page = paginator.next_page().await?;
println!("first page present: {}", first_page.is_some());
# Ok(())
# }
```

see also: [`examples/generated_api.md`](../examples/generated_api.md)

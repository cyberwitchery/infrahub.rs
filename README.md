# infrahub.rs

rust client for the infrahub graphql api.

## features

- ergonomic, typed graphql operations
- branch-aware graphql routing
- schema fetch helper
- pagination helper for connection queries
- configurable http transport (prebuilt client or builder callback)
- good errors
- examples and smoke tests

## docs

- [docs index](docs/index.md)
- [client guide](docs/client.md)
- [changelog](CHANGELOG.md)

## install

add to `Cargo.toml`:

```toml
[dependencies]
infrahub = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## quick start

```rust
use infrahub::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("http://localhost:8000", "token");
    let client = Client::new(config)?;

    let response = client
        .execute_raw("{ InfrahubInfo { deployment_id version } }", None, None)
        .await?;

    println!("response: {response:?}");
    Ok(())
}
```

## usage model

- this crate is the base graphql client
- generate a schema-specific crate with `infrahub-codegen` for ergonomic helpers

## generated client

use `infrahub-codegen` to generate a schema-specific crate. this base crate
stays stable and talks graphql directly; the generated crate adds typed,
topic-grouped helpers for your schema.

generate a schema-specific client with `infrahub-codegen`:

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

## development

update schema:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=token ./scripts/update_schema.sh
```

generate a schema-specific client:

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

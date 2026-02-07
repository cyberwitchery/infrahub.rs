# codegen

this repo ships a base client and a schema‑driven code generator. use the base
client for ad‑hoc graphql, and generate a schema‑specific crate for ergonomic
helpers.

## quick start

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

full CLI reference:

```bash
cargo run --bin infrahub-codegen -- --help
```

## fetch schema from a running infrahub

```bash
INFRAHUB_TOKEN=... cargo run --bin infrahub-codegen -- \
  --url http://localhost:8000 \
  --token $INFRAHUB_TOKEN \
  --out /tmp/infrahub-generated
```

## options

- `--schema <path>`: load schema from a file
- `--url <url>`: fetch schema from a running infrahub
- `--token <token>`: api token for schema fetch
- `--branch <name>`: optional branch for schema fetch
- `--out <path>`: output directory for the generated crate
- `--crate-name <name>`: optional crate name (defaults to directory name)
- `--infrahub-path <path>`: use a path dependency for `infrahub`

## add the generated crate

```toml
[dependencies]
infrahub = "0.0.1"
infrahub-generated = { path = "/tmp/infrahub-generated" }
```

## use the generated api

```rust,no_run
use infrahub::{Client, ClientConfig};
use infrahub_generated::ApiClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let repository_api = client.api().core().repository();
let repositories = repository_api.list(None, None).await?;
println!("repositories: {}", repositories.len());

let mut paginator = repository_api.paginate(None, None);
let _all = paginator.collect_all().await?;
# Ok(())
# }
```

for a full generated `api()` walkthrough, see:
- [`examples/generated_api.md`](../examples/generated_api.md)

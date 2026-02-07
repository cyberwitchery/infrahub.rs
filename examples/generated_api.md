# generated api example

this example shows the ergonomic `api()` layer from a generated crate.
it demonstrates list/get/paginate on a generated model client.

generate the client first:

```bash
cargo run --bin infrahub-codegen -- --schema schema/infrahub.graphql --out /tmp/infrahub-generated
```

add the generated crate as a path dependency:

```toml
[dependencies]
infrahub = "0.0.2"
infrahub-generated = { path = "/tmp/infrahub-generated" }
tokio = { version = "1", features = ["full"] }
```

example:

```rust,no_run
use infrahub::{Client, ClientConfig};
use infrahub_generated::ApiClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let repository_api = client.api().core().repository();

// list
let repositories = repository_api.list(None, None).await?;
println!("repository count: {}", repositories.len());

// get by id
if let Some(first) = repositories.first() {
    let fetched = repository_api.get_by_id(first.id.clone(), None).await?;
    println!("fetched: {}", fetched.is_some());
}

// paginate (connection-style pages)
let mut paginator = repository_api.paginate(None, None);
while let Some(page) = paginator.next_page().await? {
    println!("page size: {}", page.len());
    if page.is_empty() {
        break;
    }
}
# Ok(())
# }
```

notes:
- exact namespace/model accessors depend on your schema snapshot.
- for this repo's `schema/infrahub.graphql`, generated namespaces include `core`, `builtin`, `ipam`, `lineage`, and `profile`.
- if your schema snapshot exposes create/update helpers in `api()`, they appear on the same model client alongside `list` and `get_by_id`.

# generated api example

this example shows the ergonomic `api()` layer from a generated crate.
it demonstrates list/get/create/update with a branch-like object.

generate the client first:

```bash
cargo run --bin infrahub-codegen -- --schema schema/infrahub.graphql --out /tmp/infrahub-generated
```

add the generated crate as a path dependency:

```toml
[dependencies]
infrahub = "0.0.1"
infrahub-generated = { path = "/tmp/infrahub-generated" }
tokio = { version = "1", features = ["full"] }
```

example:

```rust,no_run
use infrahub::{Client, ClientConfig};
use infrahub_generated::ApiClient;
use infrahub_generated::inputs::{BranchCreateInput, BranchUpdateInput};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;
let branch_api = client.api().infrahub().branch();

// list
let branches = branch_api.list(None, None).await?;
println!("branch count: {}", branches.len());

// get by id
if let Some(first) = branches.first() {
    let fetched = branch_api.get_by_id(first.id.clone(), None).await?;
    println!("fetched: {}", fetched.is_some());
}

// create
let created = branch_api
    .create(
        None, // context
        BranchCreateInput {
            name: "example-doc-branch".to_string(),
            description: None,
            sync_with_git: None,
        },
        None,
    )
    .await?;

// update
let _updated = branch_api
    .update(
        None, // context
        BranchUpdateInput {
            id: created.id.clone(),
            name: Some("example-doc-branch-renamed".to_string()),
            description: None,
            sync_with_git: None,
        },
        None,
    )
    .await?;
# Ok(())
# }
```

notes:
- exact input field names depend on your schema snapshot.
- if your schema uses different branch types, adapt the input structs accordingly.

# case study: procurement contract

this example uses a custom object loaded via `extension_contract.yml`:

```yaml
nodes:
  - name: Contract
    namespace: Procurement
    attributes:
      - name: contract_ref
        kind: Text
      - name: description
        kind: Text
        optional: true
      - name: start_time
        kind: DateTime
      - name: end_time
        kind: DateTime
        optional: true
```

## typed query (concise)

```rust,no_run
use infrahub::{Client, ClientConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    #[serde(rename = "ProcurementContract")]
    procurement_contract: Contracts,
}

#[derive(Debug, Deserialize)]
struct Contracts {
    edges: Vec<Edge>,
}

#[derive(Debug, Deserialize)]
struct Edge {
    node: Option<ContractNode>,
}

#[derive(Debug, Deserialize)]
struct ContractNode {
    id: String,
    contract_ref: TextAttr,
    description: Option<TextAttr>,
}

#[derive(Debug, Deserialize)]
struct TextAttr {
    value: Option<String>,
}

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;

let query = r#"
query($ref: String) {
  ProcurementContract(contract_ref__value: $ref) {
    edges {
      node {
        id
        contract_ref { value }
        description { value }
      }
    }
  }
}
"#;

let vars = serde_json::json!({ "ref": "contract-001" });
let response = client.execute::<Data>(query, Some(vars), None).await?;

for edge in response.data.unwrap().procurement_contract.edges {
    if let Some(node) = edge.node {
        println!("{} -> {:?}", node.id, node.contract_ref.value);
    }
}
# Ok(())
# }
```

## generated client (full surface)

if you generate a full client with `infrahub-codegen`, you get a method for the custom object:

```rust,no_run
use infrahub::{Client, ClientConfig};
use infrahub_generated::GeneratedClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("http://localhost:8000", "token"))?;

let response = client
    .generated()
    .procurement_contract(
        None,        // offset
        Some(20),    // limit
        None,        // order
        None,        // ids
        None, None, None, // display_label filters
        None,        // hfid
        None, None, None, None, None, None, // start_time filters
        None, None, None, None, None, None, // description filters
        None, None, None, None, None, None, // contract_ref filters
        None, None, None, None, None, None, // end_time filters
        None, None, None, None, None, None, // any filters
        None,        // partial_match
        None, None, None, None, None, None, // node metadata filters
        None, None, None, None, None, None, // profiles filters
        None, None, None, None, None, None, // subscriber_of_groups filters
        None, None, None, None, None, None, // organization filters
        None, None, None, None, None, None, // member_of_groups filters
        None,        // request_branch
    )
    .await?;

let data = response.data.unwrap();
println!("count: {}", data.procurement_contract.count);
# Ok(())
# }
```

notes:
- generated methods include every filter arg exposed by the schema.
- for ergonomic usage, prefer the concise typed query form.
 - the generated crate also includes `api()` helpers for shorter accessors.

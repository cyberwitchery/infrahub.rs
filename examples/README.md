# examples

run examples against a local infrahub:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo run -p infrahub --example simple_query
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo run -p infrahub --example typed_query
```

examples:

- `simple_query.rs` - raw graphql query
- `typed_query.rs` - typed graphql query with ad-hoc structs
- `generated_api.md` - generated crate `api()` workflow (list/get/create/update)

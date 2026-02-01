# docs index

entrypoints:
- [client guide](client.md)
- [examples](../examples/README.md)
- [local infrahub](local-infrahub.md)
- [case studies: procurement contract](case-studies/procurement-contract.md)

how to use this repo:

- base client: `infrahub` crate with raw + typed graphql helpers
- generated client: use `infrahub-codegen` to create a schema-specific crate

developer docs:
- [dev guide](dev.md)
- [scripts](../scripts/)

quick start:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo run -p infrahub --example simple_query
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo run -p infrahub --example typed_query
```

codegen:

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

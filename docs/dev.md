# developer docs

this section covers contributor workflows and local validation steps.

## build and test

```bash
cargo build
cargo test
```

## lint and format

```bash
cargo clippy --all-targets --all-features
cargo fmt --all
```

## docs build

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps
```

open locally:

- [target/doc/infrahub/index.html](../target/doc/infrahub/index.html)

## scripts

- assurance: `./scripts/run_assurance.sh`
- static analysis: `./scripts/run_static.sh`
- smoke tests: `./scripts/run_smoke.sh`

## schema + codegen

refresh schema snapshot:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=token ./scripts/update_schema.sh
```

generate a full typed client from a schema snapshot:

```bash
cargo run --bin infrahub-codegen -- --schema /path/to/schema.graphql --out /tmp/infrahub-generated
```

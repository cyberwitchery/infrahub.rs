# local infrahub quickstart

this repo includes a docker compose file for local testing.

## setup

from the repo root:

```bash
docker compose up -d
```

## confirm it is running

ensure `http://localhost:8000` responds and you have a token.

## smoke test

run the local smoke test against your instance:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo test --test smoke_query --all-features
```

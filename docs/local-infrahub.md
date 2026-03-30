# local infrahub quickstart

## setup

from the repo root, download the upstream compose file and start infrahub:

```bash
curl -sSfL \
  "https://raw.githubusercontent.com/opsmill/infrahub/infrahub-v1.8.2/docker-compose.yml" \
  -o /tmp/infrahub-compose.yml

VERSION=1.8.2 docker compose -f /tmp/infrahub-compose.yml up -d
```

## confirm it is running

ensure `http://localhost:8000` responds and you have a token.

## smoke test

run the local smoke test against your instance:

```bash
INFRAHUB_URL=http://localhost:8000 INFRAHUB_TOKEN=... cargo test --test smoke_query --all-features
```

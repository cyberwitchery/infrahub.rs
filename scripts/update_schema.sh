#!/usr/bin/env bash
set -euo pipefail

url="${INFRAHUB_URL:-http://localhost:8000}"
branch="${INFRAHUB_BRANCH:-}"

schema_url="${url%/}/schema.graphql"
if [[ -n "$branch" ]]; then
  schema_url+="?branch=${branch}"
fi

if [[ -n "${INFRAHUB_TOKEN:-}" ]]; then
  curl -sS -H "X-INFRAHUB-KEY: ${INFRAHUB_TOKEN}" "$schema_url" -o schema/infrahub.graphql
else
  curl -sS "$schema_url" -o schema/infrahub.graphql
fi

printf 'wrote %s\n' "schema/infrahub.graphql"

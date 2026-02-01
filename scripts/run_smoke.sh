#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${INFRAHUB_TOKEN:-}" ]]; then
  echo "INFRAHUB_TOKEN is required" >&2
  exit 1
fi

INFRAHUB_URL="${INFRAHUB_URL:-http://localhost:8000}"

echo "Running infrahub smoke tests against ${INFRAHUB_URL}"

INFRAHUB_SMOKE=1 INFRAHUB_URL="${INFRAHUB_URL}" INFRAHUB_TOKEN="${INFRAHUB_TOKEN}" \
  INFRAHUB_BRANCH="${INFRAHUB_BRANCH:-}" cargo test -p infrahub --test smoke_query

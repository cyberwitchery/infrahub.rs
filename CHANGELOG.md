# changelog

## Unreleased

- ci: bump pinned Infrahub version from 1.8.5 to 1.9.0
- docs: update local-infrahub quickstart to v1.9.0
- add integration smoke tests for the generated typed API layer, exercising list / get_by_id / paginate flows against a live Infrahub instance
- add `test-client` workspace member: a pre-generated client crate from `schema/infrahub.graphql` used by integration tests
- ci: verify generated test client stays in sync with schema; run typed API smoke tests alongside existing raw-query tests

## 0.2.1 - 2026-04-26

- fix: non-JSON error responses (e.g. nginx 502 pages) now return `Error::GraphQl` with the HTTP status code preserved instead of a misleading `Error::Json`
- codegen: generated `Cargo.toml` now derives its crate version and `infrahub` dependency version from the codegen binary's own package version instead of hardcoding `0.2.0`
- add `Error::is_retryable()` for classifying transient errors (5xx, 429, timeouts, network failures) vs permanent ones (config, parse, client errors)
- return `Result::Err` instead of panicking when a file upload has an invalid MIME content type
- codegen: percent-encode the `--branch` query parameter in schema URLs, matching the library-side fix from 0.2.0
- percent-encode path segments in `graphql_url`, `file_url`, `file_by_hfid_url`, and `file_by_storage_id_url` so branch names with `/` or `#` (and other special characters in node IDs, kinds, etc.) no longer produce malformed URLs
- codegen: generated request builders now return `Result::Err` on serialization failure instead of panicking via `.expect()`
- ci: bump pinned Infrahub version from 1.8.4 to 1.8.5

## 0.2.0 - 2026-03-30

- add file upload support via the GraphQL multipart request spec (`Client::execute_multipart`, `FileUpload`)
- add file download helpers (`Client::download_file`, `download_file_by_hfid`, `download_file_by_storage_id`)
- codegen: skip `@deprecated` fields in generated types and selection sets
- codegen: handle `Upload` scalar type (maps to `Vec<u8>`)
- bump tested Infrahub version from 1.1.x to 1.8.x (CI pinned to 1.8.2)
- bump `thiserror` from 1 to 2

## 0.1.0 - 2026-02-21

- add `ClientConfig::with_http_client` to inject a prebuilt `reqwest::Client` (for proxies, custom tls, shared clients, or tracing middleware)
- add `ClientConfig::with_http_client_builder` to customize the builder after defaults are applied, without replacing the full client
- relax token validation: an empty token is now accepted when `with_http_client` is set, since auth is managed by the caller

## 0.0.2 - 2026-02-08

- codegen: switched CLI help to a shared manpage-style source used by both `--help` and rustdoc docs.
- codegen: `infrahub-codegen --help` now exits successfully (`0`) instead of erroring.
- codegen: breaking change to generated API model grouping; namespace/module grouping now uses acronym-aware identifier splitting and first-word namespace selection.
- codegen: generated `api()` model clients now include `paginate(...)` helpers returning `DynPaginator`, with default connection cursor extraction and offset fallback.
- docs: added a dedicated generated `api()` example (`examples/generated_api.md`) covering list/get and documenting schema-dependent mutation helpers.
- docs: expanded generated `api()` usage in `docs/client.md` and linked the dedicated example from `docs/codegen.md`.

## 0.0.1

- initial release

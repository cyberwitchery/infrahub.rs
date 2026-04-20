# changelog

## Unreleased

- codegen: percent-encode the `--branch` query parameter in schema URLs, matching the library-side fix from 0.2.0
- percent-encode path segments in `graphql_url`, `file_url`, `file_by_hfid_url`, and `file_by_storage_id_url` so branch names with `/` or `#` (and other special characters in node IDs, kinds, etc.) no longer produce malformed URLs

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

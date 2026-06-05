# changelog

## Unreleased

- add automatic retry with exponential backoff and jitter for transient HTTP errors (5xx, 429, timeouts, connection failures). Enabled by default (`max_retries: 3`) for `execute`, `fetch_schema`, and the file-download methods; configure via `ClientConfig::with_max_retries`, or set 0 to disable.
- tested against Infrahub 1.9.6; the `is_inherited` field was dropped from the bundled schema to match upstream (opsmill/infrahub#9146), with no change to generated client code.

## 0.3.0 - 2026-05-06

- fix: generated selection sets no longer request the `is_inherited` attribute field. Infrahub 1.9.x still advertises it in the GraphQL schema but no longer resolves it, so any query selecting it failed (tracked upstream at opsmill/infrahub#9146).
- tested against Infrahub 1.9.3 (bundled schema refreshed; local quickstart updated to 1.9.3).

## 0.2.1 - 2026-04-26

- add `Error::is_retryable()` to classify transient errors (5xx, 429, timeouts, network failures) versus permanent ones (config, parse, client errors).
- fix: non-JSON error responses (e.g. an nginx 502 page) now return `Error::GraphQl` with the HTTP status preserved, instead of a misleading `Error::Json`.
- fix: percent-encode path segments in `graphql_url`, `file_url`, `file_by_hfid_url`, and `file_by_storage_id_url`, plus the `--branch` query parameter, so branch names and node IDs containing `/` or `#` no longer produce malformed URLs.
- fix: invalid file-upload MIME content types and request-serialization failures now return `Result::Err` instead of panicking (in both the library and generated code).
- codegen: the generated `Cargo.toml` derives its crate and `infrahub` dependency versions from the codegen binary instead of hardcoding `0.2.0`.
- tested against Infrahub 1.8.5.

## 0.2.0 - 2026-03-30

- add file upload support via the GraphQL multipart request spec (`Client::execute_multipart`, `FileUpload`).
- add file download helpers (`Client::download_file`, `download_file_by_hfid`, `download_file_by_storage_id`).
- codegen: skip `@deprecated` fields, and map the `Upload` scalar to `Vec<u8>`, in generated types and selection sets.
- bump `thiserror` from 1 to 2.
- tested against Infrahub 1.8.x (CI pinned to 1.8.2).

## 0.1.0 - 2026-02-21

- add `ClientConfig::with_http_client` to inject a prebuilt `reqwest::Client` (for proxies, custom TLS, shared clients, or tracing middleware).
- add `ClientConfig::with_http_client_builder` to customize the builder after defaults are applied, without replacing the whole client.
- accept an empty token when `with_http_client` is set, since auth is then managed by the caller.

## 0.0.2 - 2026-02-08

- **breaking (codegen):** generated API model grouping now uses acronym-aware identifier splitting and first-word namespace selection.
- codegen: generated `api()` model clients now include `paginate(...)` helpers returning `DynPaginator` (connection-cursor extraction with offset fallback).
- codegen: `infrahub-codegen --help` now exits successfully instead of erroring, and shares one manpage-style help source between `--help` and the rustdoc docs.
- docs: add a generated-`api()` example (`examples/generated_api.md`) and expand the usage in `docs/client.md`.

## 0.0.1

- initial release.

# changelog

## unreleased

- add GitHub Actions integration workflow spinning up Infrahub via docker compose and running `smoke_query` tests
- add weekly upstream drift detection (`upstream-check.yml`) with automatic issue creation on new upstream releases
- add `docs/compat.md` compatibility matrix mapping client releases to tested Infrahub versions
- fix upstream-check workflow: extract pinned version from `INFRAHUB_VERSION` env var rather than image tag pattern

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

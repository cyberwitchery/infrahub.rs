# changelog

## unreleased

- codegen: switched CLI help to a shared manpage-style source used by both `--help` and rustdoc docs.
- codegen: `infrahub-codegen --help` now exits successfully (`0`) instead of erroring.
- codegen: breaking change to generated API model grouping; namespace/module grouping now uses acronym-aware identifier splitting and first-word namespace selection.
- codegen: generated `api()` model clients now include `paginate(...)` helpers returning `DynPaginator`, with default connection cursor extraction and offset fallback.
- docs: added a dedicated generated `api()` example (`examples/generated_api.md`) covering list/get and documenting schema-dependent mutation helpers.
- docs: expanded generated `api()` usage in `docs/client.md` and linked the dedicated example from `docs/codegen.md`.

## 0.0.1

- initial release

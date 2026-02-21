# compatibility

maps `infrahub` releases to the Infrahub upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| infrahub | infrahub upstream | notes                                |
|----------|-------------------|--------------------------------------|
| 0.1.x    | 1.1.x             | CI pinned to 1.1.0                   |

older client releases have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`.
when the upstream drift check opens an issue, update the pin, run CI, and add
a row here before closing the issue.

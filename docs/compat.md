# compatibility

maps `infrahub` releases to the Infrahub upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| infrahub | infrahub upstream | notes                                |
|----------|-------------------|--------------------------------------|
| main     | 1.9.x             | CI pinned to v1.9.3 (next 0.3.x)     |
| 0.3.0    | 1.9.x             | CI pinned to v1.9.3                  |
| 0.2.x    | 1.8.x             | CI pinned to v1.8.5                  |
| 0.1.x    | 1.1.x             | CI pinned to v1.1.0                  |

older client releases have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`. update the
`main` row when the pin changes; rename it to the released version when a
release is cut, then start a fresh `main` row.

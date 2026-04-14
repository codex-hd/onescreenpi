# OneScreenPI backend rebrand staging

This repo still carries a large `screenpipe` compatibility surface. The safe pass in [ONE-118](/ONE/issues/ONE-118) updates repository-facing metadata and internal package names where those values are not part of published or runtime compatibility contracts.

## Renamed in this pass

- Workspace and package repository metadata now points at `cflev/OneScreenPI`
- Internal/private package names now use `onescreenpi` where that does not affect external consumers:
  - `apps/screenpipe-app-tauri/package.json` -> `onescreenpi-app`
  - `packages/browser-extension/package.json` -> `@onescreenpi/browser-extension`
  - `packages/e2e/package.json` -> `@onescreenpi/e2e`
- Package descriptions now describe the project as OneScreenPI where the underlying published package name is intentionally unchanged

## Preserved for compatibility

- Rust crate names such as `screenpipe-engine`, `screenpipe-core`, and the `screenpipe` CLI binary
- Published npm package names under `@screenpipe/*`
- Tauri bundle identifiers, updater endpoints, deep-link schemes, and release artifact names that still target `screenpi.pe`
- Runtime storage and log paths like `~/.screenpipe`
- Existing `SCREENPIPE_*` environment variables, process names, and E2E/test helpers that match current binaries

## Why these stay for now

- Renaming crate and binary identifiers would cascade through the Rust workspace, release automation, install instructions, and downstream scripts
- Changing Tauri identifiers or updater artifact names would break auto-update continuity for existing installs
- Moving storage paths requires an explicit migration layer to preserve local data and rollback behavior
- Renaming `SCREENPIPE_*` variables would break CI, scripts, and operator tooling unless aliases are introduced first

## Recommended next stage

1. Add compatibility aliases for storage paths and environment variables before renaming runtime defaults.
2. Split release automation from the legacy `screenpi.pe` updater channel so app identity can change without breaking updates.
3. Rename Rust crate/package/binary identifiers only after the alias layer and release pipeline are in place.

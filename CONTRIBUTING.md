# Contributing to lem0x01

Thanks for helping build **lem0x01 — Linux Environment Manager**.

## What belongs where

- `crates/lem0x01-core`: state, events and orchestration primitives.
- `crates/lem0x01-protocol`: stable types shared across processes.
- `crates/lem0x01-platform`: Linux session and system-service integration.
- `crates/lem0x01-compositor`: compositor-specific adapters and capability discovery.
- `apps/lem0x01d`: long-running user daemon.
- `apps/lem0x01ctl`: command-line control surface.
- `quickshell/`: optional visual frontend.
- `docs/`: architecture and design decisions.

## Architecture rules

1. Keep the core compositor-neutral.
2. Prefer event subscriptions over polling.
3. Never make Hyprland-specific assumptions in core crates.
4. Keep IPC schemas explicit and versionable.
5. Avoid shell interpolation. Use structured process arguments.
6. Do not add root requirements for convenience.
7. Keep optional integrations behind capabilities/features.
8. Do not put system truth in a UI frontend.
9. Measure before optimizing, then keep hot paths allocation-conscious.
10. Add tests for policy and protocol changes.

## Pull requests

1. Create a focused branch: `feat/*`, `fix/*`, `perf/*`, `docs/*` or `refactor/*`.
2. Run `cargo fmt`, `cargo check --workspace` and Clippy.
3. Test on a real Wayland session when the change touches compositor/UI behavior.
4. Document security implications for process, IPC, filesystem or privilege changes.
5. Include screenshots/video for visual changes when useful.
6. Keep commits small and explain architectural changes.

## Clean-room rule

Caelestia, Impasto and other projects are inspirations/reference points. Do not copy source code. Reimplement behavior from public interfaces and documentation, respecting each project's license.

## Commit style

Prefer:

- `feat:`
- `fix:`
- `refactor:`
- `perf:`
- `docs:`
- `security:`
- `chore:`

# wem0x01 — Linux Environment Manager

**wem0x01** is a Rust-first, event-driven **Linux environment manager and Wayland session control plane**.

It is not a Quickshell configuration and it is not a desktop shell. The project coordinates the pieces that make a modern Wayland desktop environment work together: compositor integration, user-session services, devices, media, policies, profiles, themes, lifecycle, automation and optional UI frontends.

Quickshell is currently one frontend. It is intentionally not the source of truth.

## Architecture

```
                         wem0x01 frontends
             ┌──────────────┬──────────────┬──────────────┐
             │  Quickshell  │ Native Wayland│   CLI / TUI  │
             └──────────────┴───────┬──────┴──────────────┘
                                    │
                             versioned protocol
                                    │
                    ┌───────────────▼────────────────┐
                    │          wem0x01d               │
                    │   Rust control plane / daemon   │
                    │                                 │
                    │ state graph • event bus         │
                    │ capability negotiation           │
                    │ policies • profiles • lifecycle │
                    └───────────────┬────────────────┘
                                    │
             ┌──────────────────────┼──────────────────────┐
             │                      │                      │
       compositor adapters     Linux adapters          policies
       Hyprland / Sway         D-Bus / systemd          rules
       Niri / River            PipeWire / portals        profiles
       Wayfire / ...            NM / BlueZ / UPower
             │                      │
             └──────────────┬───────┴──────────────┘
                            │
                     Linux / Wayland
```

The core is deliberately compositor-neutral. A Hyprland feature is represented as a capability, not as a global assumption. This makes the same environment model usable across multiple Wayland compositors.

## Why Rust?

The control plane is intended to be long-running, security-sensitive infrastructure.

Rust gives us:
- memory-safety without a garbage collector;
- explicit ownership and predictable resource lifetimes;
- strong types across IPC/config/state boundaries;
- low overhead for an always-running user daemon;
- excellent async and systems tooling;
- a natural path to native Wayland clients through the Wayland/Smithay ecosystem.

The project does **not** force every UI to be Rust. Frontends can use the best technology for their job.

## Current components

### Rust control plane
- `wem0x01d` — user-session daemon.
- `wem0x01ctl` — control CLI.
- `wem0x01-core` — event bus and orchestration primitives.
- `wem0x01-protocol` — stable, dependency-light protocol types.
- `wem0x01-compositor` — compositor-neutral adapter boundary.
- `wem0x01-platform` — Linux/D-Bus/session integration boundary.

### Existing UI layer
The existing Quickshell frontend provides:
- dynamic top bar;
- workspaces;
- launcher;
- control center;
- notifications;
- media controls;
- system tray;
- network, Bluetooth, audio and battery surfaces;
- shared themes for Kitty, Fuzzel, Zellij and Neovim.

These are being migrated from the old `wem0x01` identity into the wem0x01 frontend architecture.

## Repository layout

```
apps/
  wem0x01d/             # control-plane daemon
  wem0x01ctl/           # CLI

crates/
  wem0x01-core/         # state/event orchestration
  wem0x01-protocol/     # versioned public protocol types
  wem0x01-platform/     # Linux session + system services
  wem0x01-compositor/   # Wayland/compositor adapters

quickshell/              # optional UI frontend
hypr/                    # Hyprland integration
kitty/ fuzzel/ nvim/     # developer environment integrations
themes/                  # shared visual tokens
config/                  # wem0x01 configuration
systemd/                 # user services
docs/                    # architecture and design docs
scripts/                 # installation/migration helpers
```

## Design principles

1. **Rust-first control plane**
2. **Event-driven, not polling-driven**
3. **Compositor-neutral capability model**
4. **Least privilege by default**
5. **No shell interpolation for controlled operations**
6. **UI is replaceable**
7. **State belongs to the daemon, not the frontend**
8. **Crash isolation between components**
9. **Atomic, versioned configuration**
10. **Observable and diagnosable**
11. **Performance is a feature**
12. **Security boundaries are architectural, not cosmetic**

## Planned subsystems

- [x] Initial Rust workspace and daemon boundary
- [x] Compositor abstraction
- [x] Stable protocol crate
- [x] User-systemd service definition
- [x] Quickshell frontend foundation
- [ ] Hyprland adapter with event-driven IPC
- [ ] Sway adapter
- [ ] Niri adapter
- [ ] River adapter
- [ ] Wayfire adapter
- [ ] Generic Wayland capability discovery
- [ ] Native D-Bus service adapters
- [ ] PipeWire/MPRIS media service
- [ ] NetworkManager/BlueZ/UPower service layer
- [ ] Profiles and policy engine
- [ ] Hotkey/action router
- [ ] OSD and notification surfaces
- [ ] Clipboard history
- [ ] Lock/idle/session lifecycle
- [ ] Screenshot/color-picker service
- [ ] Wallpaper → Material/OKLCH theme engine
- [ ] Native Rust Wayland surfaces
- [ ] Plugin/extension ABI
- [ ] Diagnostics and performance profiler
- [ ] Signed release artifacts

## Installation

The current installer still installs the existing Quickshell frontend:

    ./scripts/install.sh

For the Rust control plane:

    cargo build --release
    install -Dm755 target/release/wem0x01d ~/.local/bin/wem0x01d
    install -Dm755 target/release/wem0x01ctl ~/.local/bin/wem0x01ctl

Then install the user service:

    mkdir -p ~/.config/systemd/user
    cp systemd/wem0x01.service ~/.config/systemd/user/
    systemctl --user daemon-reload
    systemctl --user enable --now wem0x01.service

## Development

    cargo fmt --all -- --check
    cargo check --workspace
    cargo clippy --workspace --all-targets --all-features -- -D warnings

The Quickshell frontend can be run separately while the Rust control plane evolves.

## Themes

    ~/.config/wem0x01/theme-sync caelestia
    ~/.config/wem0x01/theme-sync impasto
    ~/.config/wem0x01/theme-sync ultimate

Caelestia and Impasto are visual references only. wem0x01 is a clean-room implementation and does not copy their source code.

## Security model

wem0x01 is designed to run as a **user service**, not as root.

The architecture favors:
- explicit capability grants;
- structured process spawning;
- fail-closed policy decisions;
- isolated adapters;
- minimal filesystem access;
- no ambient privilege escalation;
- auditable IPC boundaries.

See `SECURITY.md` and `docs/architecture.md`.

## Project identity

The project is evolving from its original Quickshell prototype into a broader Linux environment manager. During the v8 migration, legacy paths such as `wem0x01` remain temporarily for compatibility.

The target project name is:

**wem0x01 — Linux Environment Manager**

See `docs/architecture.md` for the long-term architecture.


## Architecture

The project is a Rust-first control plane, not a Quickshell configuration. The daemon owns state, policy and orchestration; Quickshell is an optional frontend.

See [docs/architecture.md](docs/architecture.md) and [docs/ROADMAP.md](docs/ROADMAP.md).

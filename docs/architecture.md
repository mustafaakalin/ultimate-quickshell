# lem0x01 Architecture

lem0x01 is a **Linux environment control plane for Wayland sessions**, not a desktop shell.

## Layer model

```
                 ┌──────────────────────────────────────┐
                 │             User interfaces           │
                 │ Quickshell │ Native Wayland │ CLI/TUI │
                 └───────────────────┬──────────────────┘
                                     │
                              protocol / IPC
                                     │
                 ┌───────────────────▼──────────────────┐
                 │          lem0x01 control plane       │
                 │ state graph • event bus • policies   │
                 │ lifecycle • profiles • capabilities  │
                 └───────────────┬───────────────┬──────┘
                                 │               │
                   ┌─────────────▼───┐     ┌────▼─────────────┐
                   │ Compositor layer │     │ Linux platform    │
                   │ Hyprland / Sway  │     │ D-Bus / portals   │
                   │ Niri / River ... │     │ PipeWire / systemd│
                   └─────────────┬───┘     └────┬─────────────┘
                                 │               │
                          Wayland / IPC      kernel/userspace
```

## Design rules

1. **Rust first.** The daemon and policy engine are Rust. UI languages are implementation details.
2. **Event driven.** Prefer Wayland events, D-Bus signals, compositor IPC events and file notifications over polling.
3. **Capability based.** Components advertise capabilities; the core never assumes Hyprland-only behavior.
4. **Least privilege.** The daemon runs as the user. Privileged operations are explicit adapters and fail closed.
5. **No shell interpolation.** External processes use structured argument vectors and strict allowlists.
6. **Crash isolation.** UI failures must not take down the control plane.
7. **State is authoritative.** UIs subscribe to snapshots/events rather than owning system truth.
8. **Composable.** Every integration is replaceable without changing the protocol crate.
9. **Portable.** Wayland is the common transport; compositor-specific features are negotiated.
10. **Observable.** Structured logs, health state and diagnostics are first-class.

## Planned subsystems

- compositor adapters: Hyprland, Sway, Niri, River, Wayfire;
- session/lifecycle: systemd --user, logind, portals;
- media: PipeWire + MPRIS;
- devices: NetworkManager, BlueZ, UPower;
- surfaces: bar, launcher, OSD, notifications, control center, lock, picker;
- automation: profiles, rules, hotkeys and workspace policies;
- theme engine: wallpaper → color quantization → Material/OKLCH tokens;
- state persistence: versioned local state with atomic writes;
- plugin boundary: versioned capabilities rather than arbitrary in-process plugins.

## Why not make a compositor?

lem0x01 integrates with compositors. It does not compete with them. Smithay remains an option for future native Wayland components, while existing compositors retain responsibility for rendering and window management.

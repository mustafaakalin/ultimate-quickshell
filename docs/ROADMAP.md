# wem0x01 Roadmap

## Phase 1 — Foundation
- [x] Rust workspace
- [x] State store
- [x] Typed protocol
- [x] Event bus
- [x] Deny-by-default policy primitive
- [x] User systemd service
- [x] Hyprland event adapter
- [ ] Authenticated local IPC
- [ ] Capability broker with peer identity
- [ ] Supervised actor runtime
- [ ] Bounded event journal and replay

## Phase 2 — Platform graph
- [ ] systemd user lifecycle actor
- [ ] D-Bus session actor
- [ ] NetworkManager actor
- [ ] BlueZ actor
- [ ] UPower actor
- [ ] PipeWire actor
- [ ] MPRIS actor
- [ ] portal integration

## Phase 3 — Compositor portability
- [ ] Hyprland full adapter
- [ ] Sway
- [ ] Niri
- [ ] River
- [ ] Wayfire
- [ ] standardized Wayland capability layer

## Phase 4 — Environment services
- [ ] profiles
- [ ] rules engine
- [ ] hotkey router
- [ ] notification service
- [ ] OSD
- [ ] clipboard
- [ ] screenshot/color picker
- [ ] lock/idle/session lifecycle
- [ ] wallpaper/theme engine

## Phase 5 — Native frontends
- [ ] Quickshell frontend backed entirely by wem0x01 state
- [ ] native Rust Wayland surfaces
- [ ] CLI/TUI
- [ ] remote automation client

## Phase 6 — Extension platform
- [x] modular plugin manifest and capability model
- [ ] capability-scoped extensions
- [ ] signed extension metadata
- [ ] compatibility/version negotiation

## Phase 7 — Intelligence and recovery
- [ ] Incident model and flight recorder
- [ ] Evidence bundle and redaction engine
- [ ] Diagnostic skill format
- [x] MCP capability broker model
- [x] Agent plan/approval/transaction flow
- [x] Checkpoint and rollback engine
- [ ] Agent sandbox runner
- [ ] Deterministic diagnosis/replay harness

## Phase 8 — Performance/security hardening
- [ ] startup benchmarks
- [ ] idle CPU/RSS benchmarks
- [ ] IPC latency benchmarks
- [ ] fuzz protocol/config parsers
- [ ] syscall/resource sandboxing
- [ ] security audit


## Intelligence Plane

- [x] AI asset model: agents, tools, skills, MCP servers, memory, specs
- [x] Local/cloud/hybrid agent descriptors
- [x] bounded subagent delegation model
- [x] typed memory with provenance/trust/sensitivity
- [x] governance graph
- [x] AI policy model
- [x] execution budgets
- [x] trace/evaluation primitives
- [x] namespace-scoped memory repository model
- [ ] persistent encrypted storage backend
- [ ] MCP client/server runtime
- [ ] local model runtime adapter
- [ ] cloud model adapter with explicit egress policy
- [ ] tool execution broker
- [ ] spec validator and acceptance-test runner
- [ ] skill compiler/loader
- [ ] human approval service (runtime/UI)
- [ ] OpenTelemetry exporter
- [x] prompt/context firewall and injection-resistant context handling
- [ ] AI incident diagnosis runtime
- [ ] subagent supervisor and quarantine
- [ ] governance graph persistence and impact analysis

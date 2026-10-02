# Competitive Architecture Analysis

Research snapshot: October 2026. This is an architectural comparison, not a quality ranking.

| Project | Layer | Strength | Gap vs wem0x01 |
|---|---|---|---|
| DankMaterialShell | shell + backend | Multi-compositor, Go backend, CLI/IPC, plugins | Still a shell product and its backend is tightly coupled to shell responsibilities |
| Noctalia | native Wayland shell | Cohesive shell, direct Wayland, TOML, IPC, plugins | Explicitly does not become compositor/system authority |
| Caelestia | Quickshell shell | Excellent reactive UI and compositor integration | Frontend-centric rather than a general environment control plane |
| iNiR | Quickshell shell | Deferred startup and modular shell families | Shell-centric mixed-language architecture |
| COSMIC | desktop/session | Rust components and layered session orchestration | Integrated desktop rather than universal compositor-neutral control plane |
| Omarchy | Linux desktop/distribution | Operational workflows and AI crash diagnosis | Opinionated distribution/desktop scope |
| Bluefin | immutable Linux OS | AI troubleshooting with Linux MCP tooling | OS/distribution layer rather than Wayland environment control |
| Quickshell | shell framework | Reactive UI and system integrations | Framework, not environment authority |

## What we should learn

DankMaterialShell proves the value of a backend plus CLI/IPC and a plugin system. Noctalia proves that a cohesive shell can replace many fragile desktop utilities while keeping a clear scope boundary.

COSMIC demonstrates layered Rust services and session orchestration. Niri demonstrates a particularly important IPC pattern: send a coherent initial state, then incremental events, so clients do not poll or desynchronize.

Omarchy demonstrates the most interesting AI pattern: attach AI to an operational event. A crash creates an evidence-driven workflow and invokes a purpose-built skill. Bluefin demonstrates that AI troubleshooting becomes stronger when the agent receives structured Linux tools instead of unrestricted terminal access.

## The architectural gap

Most projects split these concerns:

Wayland compositor + Linux services + shell + lifecycle + policy + diagnostics + AI

wem0x01 should make the control plane the stable boundary and keep every implementation replaceable.

Shell replaceable. Compositor replaceable. AI provider replaceable. Plugin replaceable. Linux adapter replaceable.

The stable pieces are: state model, capabilities, policy, protocol, transactions and evidence.

## The key design change

Do not grow wem0x01 into a bigger shell. Use:

events -> normalized state -> policy -> intent -> transaction -> effects -> verification -> events

The UI is an observer/controller, never the source of truth.

## Advantages to pursue

1. Control-plane-first: useful without a graphical frontend.
2. Compositor neutrality: negotiate capabilities instead of assuming them.
3. Evidence-native diagnostics: failures automatically create structured incident evidence.
4. AI as a bounded operator: agents receive capabilities and evidence, not root/shell access.
5. Transactional changes: checkpoint, apply, verify, rollback.
6. Event replay: a bounded journal makes difficult desktop failures reproducible.
7. One protocol for humans and agents: CLI, TUI, shell, automation and AI share the same API.
8. Out-of-process extensions: plugin failure cannot take down the control plane.
9. Local-first intelligence: diagnostics work without a cloud model.
10. Policy before execution: every effect is authorized before reaching an adapter.

## Disadvantages and risks

wem0x01 will be substantially more complex than a shell: multiple compositor adapters, Linux integrations, protocol compatibility, agent security, distro differences, rollback semantics and a large hardware/service test matrix.

The answer is not to remove the architecture. The answer is to make the actor the unit of integration: small, independently supervised, restartable and capability-scoped.
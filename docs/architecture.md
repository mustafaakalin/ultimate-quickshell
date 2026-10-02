# wem0x01 Architecture

wem0x01 is a Wayland Environment Manager: a user-session control plane that coordinates a Linux desktop without becoming a compositor.

## Non-negotiable design rules

### Security first
- User daemon by default; never require root for normal operation.
- Deny-by-default capability policy.
- Typed commands instead of arbitrary shell strings.
- Structured process spawning; no shell interpolation.
- Privileged effects are separate capabilities.
- Frontends are untrusted clients, not system authorities.
- Configuration and state writes are atomic.
- Every adapter has a narrow permission surface.

### Performance first
- Event-driven architecture; polling is a fallback, not a default.
- Immutable snapshots shared with Arc.
- Bounded event channels for backpressure.
- Avoid global locks on hot paths.
- Separate slow I/O from state publication.
- Lazy-start optional subsystems.
- Do not wake the daemon for UI animation.
- Measure allocations, wakeups, CPU time and IPC latency.

### Customization first
- Everything optional is a capability.
- Profiles can replace policy, theme, compositor behavior and service activation.
- UI is replaceable.
- Compositor adapters are replaceable.
- Service adapters are replaceable.
- Configuration is declarative and versioned.
- No feature should require forking the core.

## Control-plane model

Clients -> authenticated IPC -> wem0x01d -> state/policy/capability/orchestration -> adapters -> Linux/Wayland.

Clients include Quickshell, future native Wayland UIs, CLI/TUI and external automation.

## State authority

External event -> adapter -> typed event -> state reducer -> immutable snapshot -> event bus -> clients.

This prevents every UI from independently polling NetworkManager, PipeWire, Hyprland, UPower, and other services.

## Capability model

Examples:
- compositor.workspace
- compositor.window_rules
- network.scan
- network.connect
- bluetooth.control
- audio.route
- power.profile
- session.lock
- screen.capture
- process.spawn
- config.write

Capabilities are discovered at runtime. Commands require explicit effect authorization.

## Security boundary

client -> identity -> protocol validation -> capability lookup -> policy authorization -> adapter -> OS operation.

No frontend gets an implicit ability to execute arbitrary commands.

## Actor model

Long-lived integrations should become independently supervised actors. A broken optional adapter must not kill the control plane.

## Compositor strategy

The core is compositor-neutral.

Initial adapters:
- Hyprland
- Sway
- Niri
- River
- Wayfire

Wayland protocols are preferred for portable behavior. Native compositor IPC is used only where no standardized Wayland protocol exists.

## UI strategy

Quickshell remains a high-level frontend for rapid desktop surfaces. Future frontends can include native Rust/Wayland surfaces, GTK/libadwaita clients, CLI/TUI and external automation clients.

The core must never depend on Quickshell.

## Plugin strategy

Do not start with arbitrary dynamic libraries.

First-generation extensions should be out-of-process, capability-scoped, versioned and communicating through the stable protocol. A future ABI is introduced only after the protocol is mature.

## Performance budget

Target near-zero CPU while idle, no periodic polling where signals/events exist, bounded memory growth, bounded IPC queues and lazy activation for expensive integrations.

Benchmarks will cover startup, idle CPU, RSS, event-to-client latency, reconnect latency and burst handling.

## Configuration

Configuration layers:

defaults -> system/user profile -> machine/session profile -> active environment profile -> temporary runtime override.

Higher layers override lower layers without mutating source configuration.

## Long-term goal

wem0x01 is a reusable environment platform, not a monolithic desktop shell. The same logical environment should work across different compositors, frontends, themes, automation systems and hardware profiles.


## Next-generation control-plane model

wem0x01 is organized into five cooperating planes:

1. State plane — normalized immutable state and bounded event journal.
2. Policy plane — identity, capabilities, authorization and safety policy.
3. Execution plane — supervised actors and typed adapters for compositor/Linux services.
4. Transaction plane — checkpoints, previews, apply/verify/rollback for mutating operations.
5. Intelligence plane — diagnostics, evidence graphs and optional AI agents.

The planes are deliberately separated. An AI failure cannot corrupt state. A compositor adapter cannot bypass policy. A frontend cannot directly execute effects.

### Incident and evidence model

Operational failures become structured incidents rather than unstructured log dumps. The daemon maintains a bounded flight recorder and can assemble redacted evidence bundles for diagnostics.

This enables:

event -> incident -> evidence -> diagnosis -> plan -> approval -> transaction -> verification

### AI agent boundary

AI agents are ordinary protocol clients with additional capability restrictions. They may investigate automatically, but mutation flows through the same capability broker used by human clients.

No agent receives ambient shell access.

### Transaction semantics

A mutation is successful only when its postconditions are verified. A transaction can therefore be previewed, approved, applied, verified and rolled back.

This is especially important for AI-generated changes.

### Event journal and replay

The event bus should eventually have a bounded journal with sequence numbers and retention policies. A diagnostic session can replay the relevant event window into a test reducer without touching the real desktop.

This gives wem0x01 a path toward deterministic reproduction of otherwise intermittent desktop failures.

### Agent interoperability

The intelligence plane should support local models, remote LLMs, coding agents and MCP clients without making any one provider part of the core. Skills are versioned documents describing evidence requirements, allowed tools, safety constraints and verification strategy.

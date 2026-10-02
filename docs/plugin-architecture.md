# wem0x01 Plugin Architecture

wem0x01 uses a **modular-monolith core with capability-isolated external extensions**.

## Two execution tiers

### Tier A — Built-in modules

Built-in modules are Rust modules linked into `wem0x01d`.
They share the immutable state graph, event journal, reducer, transaction engine, capability broker, supervision/runtime, and typed protocol.

Typical modules:
- compositor adapters
- systemd/session integration
- PipeWire
- NetworkManager
- BlueZ
- power management
- notifications
- media/MPRIS
- configuration
- diagnostics
- transaction engine

### Tier B — External plugins

Third-party or high-risk extensions run as separate processes. They communicate through the same stable IPC protocol and capability broker.

Examples: custom UI backends, AI diagnostic providers, automation engines, hardware integrations, experimental integrations, and user-created tools.

A plugin never receives ambient daemon authority.

## Plugin manifest

A plugin declares `id`, `version`, `api_version`, `kind`, required capabilities, provided services, and dependencies.
The daemon validates every declared capability against the central capability registry. Unknown capabilities are rejected before activation.

## Why this is better than a traditional plugin loader

wem0x01 intentionally does **not** make arbitrary `.so` loading the primary extension mechanism. Dynamic in-process libraries create a large blast radius: a plugin bug can corrupt or crash the entire control plane.

Preferred model:

~~~text
                    wem0x01d
                       |
              Capability Broker
                       |
        +--------------+--------------+
        |              |              |
     Built-in       Built-in       External
      module         module         plugin
      (fast)         (fast)        (isolated)
                                      |
                                  typed IPC
~~~

## Capability model

A plugin asks for named capabilities, not arbitrary operations. The broker maps each capability to an effect class:

- `Read`
- `Control`
- `SpawnProcess`
- `WriteConfig`
- `Privileged`

The plugin only receives the minimum grants explicitly approved by policy.

## Lifecycle

~~~text
discover → parse manifest → validate API → validate dependencies
        → validate capabilities → authorize → spawn/start
        → health check → active → supervise → restart / quarantine
~~~

A failing plugin must not destabilize the daemon.

## AI plugins

AI is deliberately treated as a plugin/client class, not as privileged daemon logic.

~~~text
AI diagnostic plugin
       ↓
incident evidence
       ↓
read-only investigation
       ↓
typed transaction plan
       ↓
capability broker
       ↓
user approval when required
       ↓
transaction engine
       ↓
verify
~~~

An AI plugin cannot bypass the transaction engine by asking for a shell.

## Performance strategy

The hot path stays in-process:

~~~text
Wayland event → adapter → normalized event → reducer → state snapshot → UI subscription
~~~

External plugins only cross the IPC boundary for operations that benefit from isolation. This avoids turning every event into an IPC message.

## Security strategy

Current IPC is a bootstrap trust boundary: Unix socket, mode `0600`, peer UID verification, protocol version, bounded frames, explicit client role, and capability authorization.

The declared client role is **not** cryptographic process identity. A future hardened mode will add distinct principals/grants and, where useful, separate capability-scoped sockets.

That distinction is intentional: same-user processes are currently inside the local user trust boundary.

## Design rule

> Modules own capabilities. Plugins consume capabilities. The daemon owns authority.
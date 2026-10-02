# AI Agent Architecture

## Principle

AI is an advisor and bounded operator, never an ambient root shell.

Agent -> Intent -> Capability Broker -> Plan -> Approval -> Transaction -> Adapter -> Verify

The agent never receives arbitrary shell execution, unrestricted filesystem/network access, implicit privilege escalation or a way to bypass wem0x01 policy.

## 1. Diagnostic Plane

Maintain a small privacy-aware flight recorder: compositor events, systemd transitions, service failures, PipeWire/NetworkManager/BlueZ events, config revisions, IPC errors, resource pressure, adapter reconnects and coredump references.

Do not keep unlimited logs. Keep compact structured facts and references with explicit retention limits.

Every important failure creates an Incident:

Incident = trigger + component + time window + evidence + environment snapshot + config revisions + hypotheses + proposed actions + verification

## 2. Evidence bundles

Instead of asking a user to paste logs, construct a redacted evidence bundle automatically.

Example facts: SIGSEGV, coredump available, config changed 8 minutes earlier, GPU driver version, relevant journal window, compositor IPC state, package changes and last known-good state.

The agent must distinguish confirmed facts, strong evidence, hypotheses and unknowns.

## 3. Agent lifecycle

Observe -> Investigate -> Hypothesize -> Plan -> Approve -> Apply -> Verify -> Learn

Suggested policy classes:
- observe: automatic
- diagnose: automatic
- prepare: automatic when reversible
- low-risk mutation: configurable
- high-risk mutation: explicit user approval
- privileged mutation: never silently delegated

Plans must be typed. There is no bash -c escape hatch.

## 4. MCP

Expose a scoped MCP server over the same capability broker used by CLI and UI.

Read-only tools:
wem.snapshot, wem.incidents.list, wem.incident.inspect, wem.journal.query, wem.systemd.status, wem.compositor.state, wem.network.state, wem.audio.state, wem.process.inspect, wem.config.diff, wem.health.check

Mutating tools:
wem.plan.validate, wem.transaction.preview, wem.transaction.apply, wem.transaction.rollback

MCP is not a second authorization system. It calls the same broker.

## 5. Skills

Versioned skills should define triggers, required evidence, allowed tools, safety constraints, verification checks, rollback strategy and report format.

Initial skills:
- diagnose-compositor-crash
- diagnose-systemd-failure
- diagnose-pipewire
- diagnose-network
- diagnose-bluetooth
- diagnose-gpu
- diagnose-wayland-protocol
- diagnose-high-cpu
- diagnose-memory-pressure
- diagnose-login-session
- diagnose-regression-after-update

This generalizes the useful operational pattern shown by Omarchy's diagnose-crash skill while making it portable across compositors and distributions.

## 6. Transactional AI remediation

Do not stop at diagnosis.

Example: PipeWire failure.

Plan:
1. checkpoint current audio state
2. inspect PipeWire and WirePlumber
3. restart only failed user units
4. verify default sink
5. verify application stream
6. rollback if postconditions fail

The transaction engine records before -> actions -> after -> verification.

AI therefore proposes changes, the policy engine authorizes them, adapters execute them, and verification decides whether the transaction succeeded.

## 7. Agent sandbox

Future agent runners should use systemd user scopes, restricted filesystem namespaces, no-new-privileges, capability-specific sockets, CPU/memory/time limits and network disabled by default.

The daemon remains outside the agent sandbox and acts as the policy gate.

## 8. Human UI

Do not build a generic chatbot as the primary interface.

Show: What happened; confirmed facts; investigation status; evidence count; proposed fix; risk; Preview; Apply; Rollback.

The user should understand exactly what the agent is about to change.

## 9. Privacy

Agent context passes through a redaction layer. Default-deny: passwords, tokens, SSH private keys, browser credentials, cookies, arbitrary environment variables and private documents.

## 10. Observability

Every agent action becomes a normal wem0x01 event:
agent.requested
agent.tool.called
agent.plan.created
agent.approval.requested
agent.transaction.started
agent.effect.applied
agent.verification.completed
agent.rollback.started
agent.rollback.completed

## 11. Long-term model

Human / UI / CLI / AI
        |
   Typed Intent API
        |
 Authentication
        |
 Capability Broker
        |
 +------+----------------+
 |                       |
Diagnostic Engine   Transaction Engine
 |                       |
Evidence Graph      Checkpoints
 +-----------+-----------+
             |
         State Graph
             |
        Event Journal
             |
   +---------+---------+
   |         |         |
Compositor Linux   Extension
 adapters  adapters   actors

The key property: AI is just another client of the same control plane. It is optional, replaceable and constrained by exactly the same security model as every other client.
# Security Model

wem0x01 treats the desktop environment as a local security boundary.

## Trust hierarchy

```
human / trusted local client
        |
        v
authenticated IPC
        |
        v
principal + capability broker
        |
        v
typed intent
        |
        v
policy
        |
        v
supervised adapter
        |
        v
Linux / Wayland subsystem
```

No frontend is allowed to bypass the broker.

## Local IPC

The first IPC transport is a Unix domain socket under `$XDG_RUNTIME_DIR`.

The daemon:

- creates the socket itself;
- restricts its mode to `0600`;
- checks the connecting peer UID;
- requires a protocol-version handshake;
- caps individual frames;
- never interprets request data as shell syntax.

This is deliberately local-only. Network-exposed control is a separate future feature and must not be enabled implicitly.

## Capability model

Capabilities map to typed effects:

- `Read`
- `Control`
- `SpawnProcess`
- `WriteConfig`
- `Privileged`

A principal receives explicit capabilities. Unknown capabilities are denied.

The same broker must be used by:

- Quickshell;
- native frontends;
- CLI/TUI;
- automation;
- MCP;
- AI agents.

MCP must never create a parallel authorization path.

## Agent isolation

An AI agent is not trusted merely because it runs locally.

The intended model is:

```
agent
  -> scoped identity
  -> read-only diagnostics by default
  -> typed plan
  -> policy check
  -> approval when required
  -> transaction
  -> verification
```

The agent must not receive ambient:

- root;
- arbitrary shell execution;
- unrestricted filesystem access;
- credentials;
- browser secrets;
- private keys;
- unrestricted network access.

Future agent runners should use systemd user scopes, resource limits, restricted filesystem namespaces, no-new-privileges and capability-specific sockets.

## Transactions

Mutating operations should become transactions rather than one-shot commands:

```
checkpoint
  -> preview
  -> authorization
  -> apply
  -> verify
  -> commit
       |
       +-> failure -> rollback
```

A successful process exit is not sufficient. Postconditions must be verified.

## Incident evidence

Diagnostic evidence is collected through an explicit allowlist and redaction layer.

Secrets should never be forwarded to an agent simply because they happen to be present in an environment dump or log.

## Threats considered

### Compromised frontend

A compromised shell can only exercise capabilities granted to its principal.

### Malicious local process

Socket permissions plus peer UID checking prevent unrelated users from using the control socket. Capability authorization remains required after authentication.

### Compromised AI agent

The agent can only call capabilities granted to its agent identity. Mutation is further constrained by policy and transaction approval.

### Malicious plugin

Plugins should run out-of-process. A plugin failure must not corrupt daemon state.

### Malformed client

IPC requests have bounded frames, explicit protocol versions and typed JSON decoding.

## Security invariants

1. No arbitrary shell execution from the protocol.
2. No mutation without capability authorization.
3. No privileged operation without an explicit privileged capability.
4. No AI-specific bypass around the normal policy engine.
5. No unbounded event/log retention.
6. No plugin code in the daemon address space unless there is a compelling, reviewed reason.
7. Every mutating transaction has a verification path.

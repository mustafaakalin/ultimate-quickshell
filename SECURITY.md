# Security Policy

lem0x01 is a user-session infrastructure project. Security is therefore part of the architecture.

## Reporting vulnerabilities

Please do not disclose an exploitable vulnerability in a public issue.

Until a dedicated private security channel is configured, contact the maintainer through the GitHub account `mustafaakalin` and include:

- affected component and version;
- reproduction steps;
- expected and observed behavior;
- impact;
- proposed mitigation, if available.

Never include passwords, private keys, tokens or unnecessary personal data.

## High-risk areas

Security reports are especially relevant to:

- IPC authentication and authorization;
- command execution and argument injection;
- compositor socket handling;
- D-Bus method invocation;
- privilege boundaries;
- systemd service hardening;
- configuration parsing;
- desktop-entry handling;
- theme/plugin loading;
- filesystem permissions;
- installer/update scripts.

## Security principles

- The daemon runs as the logged-in user.
- Privileged operations are opt-in and isolated.
- Unknown capabilities fail closed.
- External commands must use structured argument vectors.
- Configuration/state writes should be atomic.
- UI frontends are not trusted as the system source of truth.

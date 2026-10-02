# AI-Native Architecture

## Design target

wem0x01 is not an AI shell. It is a **Wayland Environment Control Plane with an optional local/cloud intelligence plane**.

The AI layer is a client of the control plane. It never becomes the authority.

## Mandatory AI primitives

The intelligence plane contains these first-class assets:

1. Agents
2. Subagents
3. Models
4. Tools
5. MCP servers
6. Skills
7. Memory
8. Specs
9. Evidence
10. Evaluations
11. Traces
12. Governance graph
13. Human approvals
14. Transactions
15. Policies
16. Schedules

These are assets with versions, relationships and policy boundaries, not ad-hoc JSON blobs.

## 1. Agent architecture

Agents may run:

- local
- cloud
- hybrid
- deterministic

The runtime must be provider-independent.

An agent descriptor declares:

- identity
- version
- role
- parent agent
- allowed subagents
- model/runtime
- capabilities
- skills
- memory namespaces
- execution budget
- approval requirements

### Subagents

Subagents are not free-form children.

Each subagent receives a bounded delegation:

~~~text
parent
  |
  +-- specialist
  +-- diagnostician
  +-- planner
  +-- reviewer
  +-- executor
~~~

Every delegation has:

- allowed capabilities
- allowed tools
- allowed memory namespaces
- maximum steps
- maximum tool calls
- maximum wall time
- optional token/cost budget
- expiration
- output contract

A subagent cannot escalate to its parent’s privileges.

## 2. Local / cloud / hybrid

The same agent contract works with:

~~~text
Local model
    |
    +-- Ollama / llama.cpp / future runtimes

Cloud model
    |
    +-- remote provider

Hybrid
    |
    +-- local diagnosis
    +-- cloud reasoning when explicitly permitted
~~~

Sensitive evidence should default to local processing.

Cloud execution must be an explicit policy decision, not an accidental fallback.

## 3. Memory architecture

Memory is typed:

- Working — current task context
- Episodic — what happened
- Semantic — stable facts
- Procedural — how to perform a task

Every memory item also carries trust:

- Untrusted
- Observed
- Verified
- UserProvided

This prevents an untrusted log line from silently becoming permanent trusted memory.

Memory also carries:

- namespace
- source
- sensitivity
- creation time
- expiry
- provenance

### Memory rule

> Memory is evidence, not authority.

A memory entry must never grant a capability.

Sensitive memory must be isolated by namespace and policy.

## 4. Tools

Tools are typed assets.

Each tool declares:

- input schema
- output schema
- effects
- timeout
- output-size limit
- idempotency
- reversibility
- approval requirement
- provenance
- optional MCP server

Tool calls must pass through the same capability broker as every other mutation.

The model never receives direct shell authority.

## 5. MCP

MCP is an integration protocol, not an authorization layer.

wem0x01 treats an MCP server as a governed asset:

~~~text
Agent
  ↓
MCP client
  ↓
Capability Broker
  ↓
MCP server
  ↓
Tool
~~~

MCP tools inherit policy from wem0x01.

An MCP server cannot bypass local authorization.

IBM also describes MCP as a standardized integration layer for agents and tools rather than an agent orchestration framework. citeturn0search9

## 6. Skills

A skill is a versioned operational procedure.

A skill declares:

- trigger
- purpose
- tools
- memory
- ordered steps
- approvals
- verification
- rollback

Examples:

- diagnose-compositor-crash
- diagnose-pipewire
- diagnose-network
- diagnose-gpu
- diagnose-login
- diagnose-update-regression
- optimize-memory-pressure

Skills are executable specifications, not prompt snippets.

## 7. Spec-driven development

Every substantial mutation or automation should originate from a Spec.

~~~text
Intent
  ↓
Spec
  ↓
Validate
  ↓
Plan
  ↓
Evaluate
  ↓
Approve
  ↓
Transaction
  ↓
Verify
~~~

A spec contains:

- goal
- constraints
- invariants
- acceptance tests
- required capabilities
- allowed tools

The system should reject a plan that violates mandatory invariants.

This allows AI to generate plans without allowing AI to redefine system safety.

## 8. Governance graph

The intelligence plane maintains relationships between:

~~~text
Agent
 ├── uses → Model
 ├── uses → Skill
 ├── calls → Tool
 ├── connects → MCP Server
 ├── reads → Memory Namespace
 ├── governed-by → Policy
 ├── verified-by → Evaluation
 └── constrained-by → Spec
~~~

This is deliberately a graph rather than a flat registry.

It enables impact analysis:

> "If I disable this MCP server, which agents, skills and tools become unavailable?"

and:

> "Which agent can reach this privileged capability?"

IBM's 2026 governance direction similarly emphasizes connected visibility across agents, models, MCP servers, tools, risks and controls rather than isolated asset inventories. citeturn0search5turn0search8

## 9. Continuous evaluation

Agent execution must produce evaluation evidence.

Evaluation targets include:

- correctness
- tool-call validity
- policy violations
- unsafe actions
- hallucination indicators
- latency
- resource consumption
- rollback success
- task success

Evaluation can happen:

- offline before deployment
- online during execution
- after incidents
- during regression tests

IBM's current governance material explicitly emphasizes agent/tool evaluation and enforcement evidence rather than relying only on static policy documents. citeturn0search2turn0search4

## 10. Observability

Every agent run gets a trace:

~~~text
trace
 ├── user intent
 ├── spec
 ├── agent
 ├── subagents
 ├── memory reads
 ├── model calls
 ├── tool calls
 ├── MCP calls
 ├── policy decisions
 ├── approvals
 ├── transactions
 ├── verification
 └── final outcome
~~~

This is essential for debugging and security auditing.

The trace should use OpenTelemetry-compatible concepts where practical.

## 11. Human-in-the-loop

Approval is a first-class state, not a UI-only feature.

~~~text
PLANNED
  ↓
AWAITING_APPROVAL
  ↓
APPROVED / DENIED
  ↓
EXECUTING
~~~

An approval authorizes exactly the displayed transaction scope.

Changing the plan invalidates the approval.

## 12. AI safety boundary

The strongest rule:

> AI can propose authority; only policy can grant authority.

Therefore:

- no arbitrary shell
- no implicit sudo
- no credential access
- no unrestricted filesystem
- no unrestricted network
- no hidden capability escalation
- no AI-specific authorization bypass
- no tool that silently changes its declared effects

## 13. Performance architecture

AI must not sit on the desktop hot path.

Bad:

~~~text
Wayland event → AI → state update
~~~

Good:

~~~text
Wayland event → adapter → reducer → state
                              |
                              +→ async evidence stream → AI
~~~

The desktop control plane remains deterministic and low-latency.

AI runs asynchronously with budgets and backpressure.

## 14. Failure isolation

Agent failures must not crash the daemon.

External agents/plugins are supervised.

Repeated failures lead to quarantine.

AI timeouts are bounded.

Tool output is bounded.

Memory growth is bounded.

MCP calls are bounded.

Recursive subagent spawning is bounded.

## 15. Cost and resource governance

Every agent can have:

- CPU budget
- memory budget
- wall-clock budget
- tool-call budget
- token budget
- cloud-cost budget

A runaway agent becomes a resource-management problem, not an infinite loop.

## 16. AI control-plane architecture

~~~text
                     USER
                       |
                  Intent / Spec
                       |
                Policy + Planner
                       |
          +------------+-------------+
          |                          |
       Local Agent              Cloud Agent
          |                          |
          +------------+-------------+
                       |
                  Agent Runtime
                       |
                Subagent Manager
                       |
              +--------+--------+
              |        |        |
           Memory    Skills    Tools
                       |        |
                       +--- MCP +
                           |
                   Capability Broker
                           |
                    Transaction Engine
                           |
                    Wayland / Linux
~~~

## 17. Non-negotiable security rules

1. Capability broker is the single authorization point.
2. MCP never becomes an authorization bypass.
3. Memory never grants authority.
4. Skills never bypass policy.
5. Subagents never inherit unlimited parent authority.
6. Cloud models never receive sensitive context unless explicitly permitted.
7. Tool schemas and effects are versioned.
8. Transaction approvals are scope-bound.
9. Every mutation has evidence and verification.
10. External plugins are isolated.
11. AI cannot invoke arbitrary shell syntax.
12. Every important AI action is traceable.

## 18. IBM-derived architectural lessons

IBM's recent agentic control-plane work emphasizes centralized visibility, policy enforcement, access visibility, reusable agent catalogs, scheduling, traces and operational analytics. citeturn0search0turn0search10

IBM's governance work additionally highlights continuous discovery of agents, tools and MCP servers, connected governance relationships, runtime enforcement evidence and proactive risk assessment. citeturn0search1turn0search4

For wem0x01, these become local-first primitives rather than cloud-only services.

## Final principle

> **wem0x01 should be able to operate perfectly without AI, but become dramatically more capable when AI is connected.**

AI is an extension of the control plane.

The control plane is never an extension of AI.

## Runtime security invariants

The implementation must preserve these invariants:

- AI context is never treated as executable authority.
- Untrusted memory cannot grant capabilities.
- Tool input/output is bounded.
- Tool effects are declared before execution.
- Remote/cloud agents are denied sensitive context by default.
- Subagents operate under explicit bounded delegation.
- Mutation requires a transaction.
- Approval is invalidated if the transaction scope changes.
- Every transaction has verification before commit.
- Failed reversible mutations must be rollback-capable.
- External plugins are process-isolated.
- Same-user IPC identity is still a bootstrap trust boundary until scoped credentials are implemented.

## Performance invariants

The AI plane must never block the desktop state hot path.

- state reduction remains synchronous/deterministic where possible
- AI work is asynchronous
- event queues are bounded
- tool calls have timeouts
- model calls have budgets
- memory retrieval has limits
- plugin crashes are isolated
- expensive AI operations consume dedicated worker capacity

## Next runtime layers

1. Tool execution broker
2. MCP runtime
3. Agent orchestrator
4. Subagent supervisor
5. encrypted persistent memory
6. local model adapter
7. explicit cloud egress adapter
8. spec validator
9. skill loader
10. approval service
11. evaluation runner
12. OpenTelemetry exporter

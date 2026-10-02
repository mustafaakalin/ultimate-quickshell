//! Stable wire types. Keep this crate small so every frontend can consume it.

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;

/// Declared client role. Authorization is still enforced by the daemon and
/// must not trust this field as process identity; it is a protocol-level hint
/// until distinct principals/sockets are introduced.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ClientKind {
    Ui,
    Cli,
    Agent,
    Plugin,
    Automation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSnapshot {
    pub protocol: u16,
    pub compositor: String,
    pub session_id: String,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Reload,
    SetProfile(String),
    OpenSurface(String),
    CloseSurface(String),
    RestartComponent(String),
    Agent(AgentIntent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorEvent {
    pub compositor: String,
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub component: String,
    pub severity: IncidentSeverity,
    pub trigger: String,
    pub timestamp_unix_ms: u64,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentIntent {
    InvestigateIncident { incident_id: String },
    DiagnoseIncident { incident_id: String },
    PreviewTransaction { transaction_id: String },
    ApplyTransaction { transaction_id: String },
    RollbackTransaction { transaction_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAction {
    pub capability: String,
    pub operation: String,
    pub reversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPlan {
    pub id: String,
    pub reason: String,
    pub actions: Vec<TransactionAction>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcHello {
    pub protocol: u16,
    pub client: String,
    #[serde(default)]
    pub kind: Option<ClientKind>,
    pub requested_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub hello: Option<IpcHello>,
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub ok: bool,
    pub error: Option<String>,
    pub snapshot: Option<EnvironmentSnapshot>,
}

pub const IPC_MAX_FRAME: usize = 64 * 1024;

//! Stable wire types. Keep this crate small so every frontend can consume it.

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;

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


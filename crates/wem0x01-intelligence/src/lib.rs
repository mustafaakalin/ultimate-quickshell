//! AI-native intelligence plane for wem0x01.
//!
//! The intelligence plane is deliberately decoupled from the daemon core.
//! Agents, tools, skills, memory and specs are registered assets. Execution
//! still crosses the core capability broker and transaction boundary.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub mod agent;
pub mod graph;
pub mod memory;
pub mod mcp;
pub mod registry;
pub mod skill;
pub mod spec;
pub mod tool;

pub use agent::{AgentBudget, AgentDescriptor, AgentKind, AgentRole, AgentRuntime};
pub use graph::{AssetKind, GraphEdge, GraphNode, GovernanceGraph, Relation};
pub use memory::{MemoryEntry, MemoryKind, MemoryStore, MemoryTrust};
pub use mcp::{McpServerDescriptor, McpTransport};
pub use registry::{AssetId, AssetRegistry, AssetRevision};
pub use skill::{SkillDescriptor, SkillStep};
pub use spec::{Spec, SpecInvariant, SpecState};
pub use tool::{ToolDescriptor, ToolEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub id: String,
    pub source: String,
    pub digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub parent_id: Option<String>,
    pub agent_id: Option<String>,
    pub skill_id: Option<String>,
    pub tool_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub evaluator: String,
    pub passed: bool,
    pub score: Option<f64>,
    pub findings: Vec<String>,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Error)]
pub enum IntelligenceError {
    #[error("asset already exists: {0}")]
    AssetExists(String),
    #[error("asset not found: {0}")]
    AssetNotFound(String),
    #[error("invalid specification: {0}")]
    InvalidSpec(String),
    #[error("memory operation failed: {0}")]
    Memory(String),
    #[error("policy denied: {0}")]
    PolicyDenied(String),
}

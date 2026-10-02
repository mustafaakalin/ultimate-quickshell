//! AI-native intelligence plane for wem0x01.
//!
//! The intelligence plane is deliberately decoupled from the daemon core.
//! Agents, tools, skills, memory and specs are registered assets. Execution
//! still crosses the core capability broker and transaction boundary.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod agent;
pub mod compositor_adapter;
pub mod evaluation;
pub mod execution;
pub mod firewall;
pub mod gateway;
pub mod graph;
pub mod mcp;
pub mod memory;
pub mod memory_repo;
pub mod operation;
pub mod orchestrator;
pub mod policy;
pub mod registry;
pub mod skill;
pub mod spec;
pub mod test_adapter;
pub mod tool;
pub mod trace;

pub use agent::{AgentBudget, AgentDescriptor, AgentKind, AgentRole, AgentRuntime};
pub use compositor_adapter::{CompositorAdapter, WorkspaceFocus};
pub use evaluation::{EvaluationReport, EvaluationSuite};
pub use execution::{ActionAdapter, ExecutionBroker, ExecutionContext, ExecutionError};
pub use firewall::{ContextFirewall, ContextItem, SanitizedContext, TrustLabel};
pub use gateway::{ToolCall, ToolError, ToolGateway, ToolHandler, ToolResult};
pub use graph::{AssetKind, GovernanceGraph, GraphEdge, GraphNode, Relation};
pub use mcp::{McpServerDescriptor, McpTransport};
pub use memory::{MemoryEntry, MemoryStore};
pub use memory_repo::{MemoryRecord, MemoryRepository};
pub use operation::{
    Idempotency, OperationEffect, OperationRegistry, OperationSpec, OperationSpecError,
};
pub use orchestrator::{AgentRun, Delegation, RunState};
pub use policy::{AiPolicy, ContextDisposition};
pub use registry::{AssetId, AssetRegistry, AssetRevision};
pub use skill::{SkillDescriptor, SkillStep};
pub use spec::{Spec, SpecInvariant, SpecState};
pub use test_adapter::NoopAdapter;
pub use tool::{ToolDescriptor, ToolEffect};
pub use trace::{Trace, TraceEvent};

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

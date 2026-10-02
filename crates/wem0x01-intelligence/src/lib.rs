//! AI-native intelligence plane for wem0x01.
//!
//! The intelligence plane is deliberately decoupled from the daemon core.
//! Agents, tools, skills, memory and specs are registered assets. Execution
//! still crosses the core capability broker and transaction boundary.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod agent;
pub mod graph;
pub mod gateway;
pub mod memory;
pub mod memory_repo;
pub mod orchestrator;
pub mod evaluation;
pub mod execution;
pub mod firewall;
pub mod policy;
pub mod trace;
pub mod mcp;
pub mod registry;
pub mod skill;
pub mod spec;
pub mod tool;
pub mod test_adapter;

pub use agent::{AgentBudget, AgentDescriptor, AgentKind, AgentRole, AgentRuntime};
pub use graph::{AssetKind, GraphEdge, GraphNode, GovernanceGraph, Relation};
pub use gateway::{ToolCall, ToolError, ToolGateway, ToolHandler, ToolResult};
pub use memory::{MemoryEntry, MemoryKind, MemoryStore, MemoryTrust};
pub use memory_repo::{MemoryRecord, MemoryRepository};
pub use mcp::{McpServerDescriptor, McpTransport};
pub use orchestrator::{AgentOrchestrator, AgentRun, ApprovalScope, Delegation, OrchestratorError, RunState};
pub use evaluation::{EvaluationReport, EvaluationSuite};
pub use execution::{ActionAdapter, ExecutionBroker, ExecutionError};
pub use test_adapter::NoopAdapter;
pub use firewall::{ContextFirewall, ContextItem, SanitizedContext, TrustLabel};
pub use policy::{AiPolicy, ContextDisposition};
pub use trace::{Trace, TraceEvent};
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

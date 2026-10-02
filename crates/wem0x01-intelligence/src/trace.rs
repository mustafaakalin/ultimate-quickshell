use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceEvent {
    Intent { text: String },
    Spec { id: String, revision: u64 },
    AgentStarted { id: String },
    SubagentDelegated { parent: String, child: String },
    MemoryRead { namespace: String, count: usize },
    ModelCall { provider: String, model: Option<String> },
    ToolCall { tool: String },
    PolicyDecision { subject: String, decision: String },
    ApprovalRequested { transaction_id: String },
    Transaction { transaction_id: String, state: String },
    Evaluation { evaluator: String, passed: bool },
    Completed { success: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub trace_id: String,
    pub parent_id: Option<String>,
    pub events: Vec<TraceEvent>,
}

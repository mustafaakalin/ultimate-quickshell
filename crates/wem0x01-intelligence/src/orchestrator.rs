use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunState {
    Planned,
    AwaitingApproval,
    Running,
    Verifying,
    Committed,
    RolledBack,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub parent_agent: String,
    pub child_agent: String,
    pub allowed_tools: Vec<String>,
    pub allowed_capabilities: Vec<String>,
    pub memory_namespaces: Vec<String>,
    pub max_steps: u32,
    pub max_tool_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRun {
    pub id: String,
    pub agent_id: String,
    pub spec_id: Option<String>,
    pub state: RunState,
    pub delegations: Vec<Delegation>,
    pub tool_calls: u32,
    pub steps: u32,
}

impl AgentRun {
    pub fn can_call_tool(&self, tool: &str) -> bool {
        self.delegations
            .iter()
            .any(|d| d.allowed_tools.iter().any(|t| t == tool))
            || self.delegations.is_empty()
    }
}

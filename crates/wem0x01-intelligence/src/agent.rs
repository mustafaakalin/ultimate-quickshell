use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentKind {
    Local,
    Cloud,
    Hybrid,
    Deterministic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntime {
    pub provider: String,
    pub model: Option<String>,
    pub endpoint: Option<String>,
    pub locality: AgentKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudget {
    pub max_wall_time_ms: u64,
    pub max_steps: u32,
    pub max_tool_calls: u32,
    pub max_tokens: Option<u64>,
    pub max_cost_microunits: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    Root,
    Specialist,
    Reviewer,
    Planner,
    Executor,
    Diagnostician,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptor {
    pub id: String,
    pub role: AgentRole,
    pub parent_agent: Option<String>,
    pub allowed_subagents: Vec<String>,
    pub version: String,
    pub runtime: AgentRuntime,
    pub capabilities: Vec<String>,
    pub skills: Vec<String>,
    pub memory_namespaces: Vec<String>,
    pub max_steps: u32,
    pub max_tool_calls: u32,
    pub requires_approval_for: Vec<String>,
    pub budget: AgentBudget,
}

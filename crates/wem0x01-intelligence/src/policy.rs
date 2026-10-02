use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextDisposition { Allow, Redact, RequireApproval, Deny }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPolicy {
    pub id: String,
    pub allowed_agents: Vec<String>,
    pub allowed_tools: Vec<String>,
    pub allowed_mcp_servers: Vec<String>,
    pub allowed_memory_namespaces: Vec<String>,
    pub allow_cloud: bool,
    pub allow_external_network: bool,
    pub require_approval_for_mutations: bool,
    pub max_subagent_depth: u16,
}

impl Default for AiPolicy {
    fn default() -> Self {
        Self {
            id: "default".into(),
            allowed_agents: vec![],
            allowed_tools: vec![],
            allowed_mcp_servers: vec![],
            allowed_memory_namespaces: vec![],
            allow_cloud: false,
            allow_external_network: false,
            require_approval_for_mutations: true,
            max_subagent_depth: 3,
        }
    }
}

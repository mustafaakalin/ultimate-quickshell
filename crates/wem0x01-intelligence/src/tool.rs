use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolEffect { Read, Mutate, Execute, Privileged, ExternalNetwork }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptor {
    pub id: String,
    pub version: String,
    pub description: String,
    pub input_schema: String,
    pub output_schema: String,
    pub effects: Vec<ToolEffect>,
    pub timeout_ms: u64,
    pub max_output_bytes: usize,
    pub idempotent: bool,
    pub reversible: bool,
    pub requires_approval: bool,
    pub mcp_server: Option<String>,
}

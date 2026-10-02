use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum McpTransport {
    Stdio,
    UnixSocket,
    Http,
    Sse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDescriptor {
    pub id: String,
    pub version: String,
    pub transport: McpTransport,
    pub endpoint: String,
    pub tools: Vec<String>,
    pub trust_domain: String,
    pub allow_network: bool,
    pub requires_approval: bool,
}

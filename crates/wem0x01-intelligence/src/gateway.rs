use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wem0x01_core::{CapabilityRegistry, PolicyEngine, Principal};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub tool: String,
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub ok: bool,
    pub output: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("tool not registered: {0}")]
    NotFound(String),
    #[error("tool capability denied: {0}")]
    Denied(String),
    #[error("tool input exceeds limit")]
    InputTooLarge,
    #[error("tool output exceeds limit")]
    OutputTooLarge,
    #[error("tool execution failed: {0}")]
    Execution(String),
}

#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    async fn call(&self, input: &str) -> Result<String, ToolError>;
}

pub struct ToolGateway {
    handlers: BTreeMap<String, Box<dyn ToolHandler>>,
    max_input_bytes: usize,
    max_output_bytes: usize,
}

impl ToolGateway {
    pub fn new(max_input_bytes: usize, max_output_bytes: usize) -> Self {
        Self {
            handlers: BTreeMap::new(),
            max_input_bytes,
            max_output_bytes,
        }
    }

    pub fn register(&mut self, id: impl Into<String>, handler: Box<dyn ToolHandler>) {
        self.handlers.insert(id.into(), handler);
    }

    pub async fn call(
        &self,
        request: &ToolCall,
        principal: &Principal,
        policy: &PolicyEngine,
        capabilities: &CapabilityRegistry,
        capability: &str,
    ) -> Result<ToolResult, ToolError> {
        if request.input.len() > self.max_input_bytes {
            return Err(ToolError::InputTooLarge);
        }
        capabilities
            .authorize(policy, principal, capability)
            .map_err(|_| ToolError::Denied(capability.into()))?;
        let handler = self
            .handlers
            .get(&request.tool)
            .ok_or_else(|| ToolError::NotFound(request.tool.clone()))?;
        let output = handler.call(&request.input).await?;
        if output.len() > self.max_output_bytes {
            return Err(ToolError::OutputTooLarge);
        }
        Ok(ToolResult {
            call_id: request.id.clone(),
            ok: true,
            output,
        })
    }
}

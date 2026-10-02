use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{ActionAdapter, ExecutionError};
use wem0x01_core::TransactionAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFocus {
    pub workspace: u32,
}

pub struct CompositorAdapter {
    backend: String,
}

impl CompositorAdapter {
    pub fn new(backend: impl Into<String>) -> Self {
        Self {
            backend: backend.into(),
        }
    }

    pub fn backend(&self) -> &str {
        &self.backend
    }
}

#[async_trait]
impl ActionAdapter for CompositorAdapter {
    fn operation_prefix(&self) -> &str {
        "compositor.workspace."
    }

    async fn execute(&self, action: &TransactionAction) -> Result<(), ExecutionError> {
        match action.operation.as_str() {
            "compositor.workspace.focus" => {
                // Typed adapter boundary. Backend-specific IPC belongs here;
                // never interpret action.operation as shell syntax.
                let _ = self.backend.as_str();
                Ok(())
            }
            other => Err(ExecutionError::AdapterNotFound(other.into())),
        }
    }

    async fn verify(&self, action: &TransactionAction) -> Result<(), ExecutionError> {
        match action.operation.as_str() {
            "compositor.workspace.focus" => Ok(()),
            other => Err(ExecutionError::Verification(format!(
                "unsupported compositor operation: {other}"
            ))),
        }
    }

    async fn rollback(&self, _action: &TransactionAction) -> Result<(), ExecutionError> {
        // Focus rollback requires the previous workspace snapshot. The
        // transaction/checkpoint layer will supply that state before this
        // becomes a mutating production adapter.
        Ok(())
    }
}

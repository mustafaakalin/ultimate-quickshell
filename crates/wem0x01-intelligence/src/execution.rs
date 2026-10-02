use async_trait::async_trait;
use std::collections::BTreeMap;

use wem0x01_core::{CapabilityRegistry, Effect, PolicyEngine, Principal, Transaction, TransactionAction, TransactionEngine, TransactionError};

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("adapter not registered: {0}")]
    AdapterNotFound(String),
    #[error("capability denied: {0}")]
    CapabilityDenied(String),
    #[error("transaction error: {0}")]
    Transaction(#[from] TransactionError),
    #[error("adapter execution failed: {0}")]
    Adapter(String),
    #[error("verification failed: {0}")]
    Verification(String),
}

#[async_trait]
pub trait ActionAdapter: Send + Sync {
    fn operation_prefix(&self) -> &str;
    async fn execute(&self, action: &TransactionAction) -> Result<(), ExecutionError>;
    async fn verify(&self, action: &TransactionAction) -> Result<(), ExecutionError>;
    async fn rollback(&self, action: &TransactionAction) -> Result<(), ExecutionError>;
}

#[derive(Default)]
pub struct ExecutionBroker {
    adapters: BTreeMap<String, Box<dyn ActionAdapter>>,
}

impl ExecutionBroker {
    pub fn register(&mut self, adapter: Box<dyn ActionAdapter>) {
        self.adapters.insert(adapter.operation_prefix().to_owned(), adapter);
    }

    fn adapter_for(&self, operation: &str) -> Option<&dyn ActionAdapter> {
        self.adapters.iter()
            .filter(|(prefix, _)| operation.starts_with(prefix.as_str()))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(_, adapter)| adapter.as_ref())
    }

    pub async fn execute(
        &self,
        tx: &Transaction,
        principal: &Principal,
        policy: &PolicyEngine,
        capabilities: &CapabilityRegistry,
        transactions: &mut TransactionEngine,
    ) -> Result<(), ExecutionError> {
        for action in &tx.actions {
            capabilities.authorize(policy, principal, &action.capability)
                .map_err(|_| ExecutionError::CapabilityDenied(action.capability.clone()))?;
        }

        transactions.begin(&tx.id)?;

        let mut completed = Vec::new();
        for action in &tx.actions {
            let Some(adapter) = self.adapter_for(&action.operation) else {
                let _ = transactions.rollback(&tx.id);
                return Err(ExecutionError::AdapterNotFound(action.operation.clone()));
            };

            if let Err(error) = adapter.execute(action).await {
                for previous in completed.iter().rev() {
                    if let Some(adapter) = self.adapter_for(&previous.operation) {
                        let _ = adapter.rollback(previous).await;
                    }
                }
                let _ = transactions.rollback(&tx.id);
                return Err(error);
            }
            completed.push(action.clone());
        }

        transactions.begin_verify(&tx.id)?;

        for action in &tx.actions {
            let adapter = self.adapter_for(&action.operation)
                .ok_or_else(|| ExecutionError::AdapterNotFound(action.operation.clone()))?;
            if let Err(error) = adapter.verify(action).await {
                for previous in completed.iter().rev() {
                    if let Some(adapter) = self.adapter_for(&previous.operation) {
                        let _ = adapter.rollback(previous).await;
                    }
                }
                let _ = transactions.rollback(&tx.id);
                return Err(match error {
                    ExecutionError::Verification(_) => error,
                    other => ExecutionError::Verification(other.to_string()),
                });
            }
        }

        transactions.commit(&tx.id)?;
        Ok(())
    }
}

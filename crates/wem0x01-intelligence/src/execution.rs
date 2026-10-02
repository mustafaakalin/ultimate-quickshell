use async_trait::async_trait;
use std::collections::BTreeMap;

use crate::OperationRegistry;
use wem0x01_core::{
    CapabilityRegistry, Checkpoint, PolicyEngine, Principal, StateStore, Transaction,
    TransactionAction, TransactionEngine, TransactionError,
};

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
/// Executes only registered typed operations and verifies them before commit.
pub struct ExecutionBroker {
    adapters: BTreeMap<String, Box<dyn ActionAdapter>>,
}

impl ExecutionBroker {
    pub fn register(&mut self, adapter: Box<dyn ActionAdapter>) {
        self.adapters
            .insert(adapter.operation_prefix().to_owned(), adapter);
    }

    fn adapter_for(&self, operation: &str) -> Option<&dyn ActionAdapter> {
        self.adapters
            .iter()
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
        operations: &OperationRegistry,
        state: &StateStore,
        transactions: &mut TransactionEngine,
    ) -> Result<(), ExecutionError> {
        for action in &tx.actions {
            capabilities
                .authorize(policy, principal, &action.capability)
                .map_err(|_| ExecutionError::CapabilityDenied(action.capability.clone()))?;
            let spec = operations
                .get(&action.operation)
                .ok_or_else(|| ExecutionError::AdapterNotFound(action.operation.clone()))?;
            if spec.capability != action.capability || spec.reversible != action.reversible {
                return Err(ExecutionError::Adapter(format!(
                    "operation contract mismatch: {}",
                    action.operation
                )));
            }
            if action.reversible
                && spec.rollback_operation.as_deref() != action.rollback_operation.as_deref()
            {
                return Err(ExecutionError::Adapter(format!(
                    "rollback contract mismatch: {}",
                    action.operation
                )));
            }
        }

        if transactions.checkpoint_for(&tx.id).is_none() {
            let checkpoint_id = format!("cp-{}", tx.id);
            let snapshot = state.snapshot().await;
            let digest = snapshot
                .digest()
                .map_err(|error| ExecutionError::Adapter(error.to_string()))?;
            let checkpoint = Checkpoint::new(
                checkpoint_id.clone(),
                tx.id.clone(),
                snapshot.generation,
                digest,
            );
            transactions.checkpoint(checkpoint)?;
            transactions.attach_checkpoint(&tx.id, checkpoint_id)?;
        }
        transactions.begin(&tx.id)?;

        let mut completed: Vec<TransactionAction> = Vec::new();
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
            let adapter = self
                .adapter_for(&action.operation)
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

#[cfg(test)]
mod tests {
    use super::*;
    use wem0x01_core::{Effect, TransactionState};

    #[tokio::test]
    async fn executes_validated_transaction_through_adapter() {
        let mut capabilities = CapabilityRegistry::default();
        capabilities.register("test.noop", Effect::Control);

        let policy = PolicyEngine;
        let principal = Principal::frontend("test");
        let action = TransactionAction {
            capability: "test.noop".into(),
            operation: "test.noop.reset".into(),
            reversible: true,
            rollback_operation: Some("test.noop.rollback".into()),
        };
        let tx = Transaction {
            id: "tx-1".into(),
            principal: principal.id.clone(),
            reason: "integration test".into(),
            state: TransactionState::Draft,
            actions: vec![action],
            preconditions: vec![],
            postconditions: vec![],
            requires_approval: false,
            approval_revision: 0,
            checkpoint_id: None,
        };

        let mut transactions = TransactionEngine::default();
        transactions.insert(tx.clone());
        transactions
            .validate("tx-1", &principal, &policy, &capabilities)
            .unwrap();

        let mut broker = ExecutionBroker::default();
        let mut operations = OperationRegistry::default();
        operations
            .register(crate::OperationSpec {
                id: "test.noop.reset".into(),
                version: 1,
                capability: "test.noop".into(),
                effect: crate::OperationEffect::Control,
                input_schema: "none".into(),
                output_schema: "ack".into(),
                timeout_ms: 1000,
                max_input_bytes: 1024,
                max_output_bytes: 1024,
                idempotency: crate::Idempotency::Idempotent,
                reversible: true,
                rollback_operation: Some("test.noop.rollback".into()),
                verification_operation: None,
            })
            .unwrap();
        broker.register(Box::new(crate::NoopAdapter));

        broker
            .execute(
                &tx,
                &principal,
                &policy,
                &capabilities,
                &operations,
                &StateStore::new(Default::default()),
                &mut transactions,
            )
            .await
            .unwrap();

        assert_eq!(
            transactions.get("tx-1").unwrap().state,
            TransactionState::Committed
        );
    }
}

use async_trait::async_trait;
use super::{ActionAdapter, ExecutionError};
use wem0x01_core::TransactionAction;

/// A deliberately inert adapter used as the first integration-test boundary.
/// Real compositor/system adapters must replace this with typed operations;
/// no adapter is allowed to interpret an operation as shell syntax.
pub struct NoopAdapter;

#[async_trait]
impl ActionAdapter for NoopAdapter {
    fn operation_prefix(&self) -> &str { "test.noop." }

    async fn execute(&self, _action: &TransactionAction) -> Result<(), ExecutionError> {
        Ok(())
    }

    async fn verify(&self, _action: &TransactionAction) -> Result<(), ExecutionError> {
        Ok(())
    }

    async fn rollback(&self, _action: &TransactionAction) -> Result<(), ExecutionError> {
        Ok(())
    }
}

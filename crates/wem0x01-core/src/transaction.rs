use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::{CapabilityRegistry, PolicyEngine, Principal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionState {
    Draft, Validated, AwaitingApproval, Approved, Executing,
    Verifying, Committed, RollingBack, RolledBack, Failed, Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precondition {
    pub id: String,
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postcondition {
    pub id: String,
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAction {
    pub capability: String,
    pub operation: String,
    pub reversible: bool,
    pub rollback_operation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub principal: String,
    pub reason: String,
    pub state: TransactionState,
    pub actions: Vec<TransactionAction>,
    pub preconditions: Vec<Precondition>,
    pub postconditions: Vec<Postcondition>,
    pub requires_approval: bool,
    pub approval_revision: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("transaction not found: {0}")]
    NotFound(String),
    #[error("invalid transaction state")]
    InvalidState,
    #[error("capability denied: {0}")]
    CapabilityDenied(String),
    #[error("transaction requires approval")]
    ApprovalRequired,
    #[error("non-reversible action prevents automatic rollback")]
    NotReversible,
}

#[derive(Default)]
pub struct TransactionEngine {
    transactions: BTreeMap<String, Transaction>,
}

impl TransactionEngine {
    pub fn insert(&mut self, tx: Transaction) { self.transactions.insert(tx.id.clone(), tx); }

    pub fn get(&self, id: &str) -> Option<&Transaction> { self.transactions.get(id) }

    pub fn validate(
        &mut self,
        id: &str,
        principal: &Principal,
        policy: &PolicyEngine,
        capabilities: &CapabilityRegistry,
    ) -> Result<(), TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !matches!(tx.state, TransactionState::Draft) { return Err(TransactionError::InvalidState); }

        for action in &tx.actions {
            capabilities.authorize(policy, principal, &action.capability)
                .map_err(|_| TransactionError::CapabilityDenied(action.capability.clone()))?;
        }

        tx.state = if tx.requires_approval {
            TransactionState::AwaitingApproval
        } else {
            TransactionState::Approved
        };
        Ok(())
    }

    pub fn approve(&mut self, id: &str) -> Result<u64, TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !matches!(tx.state, TransactionState::AwaitingApproval) { return Err(TransactionError::InvalidState); }
        tx.approval_revision = tx.approval_revision.saturating_add(1);
        tx.state = TransactionState::Approved;
        Ok(tx.approval_revision)
    }

    pub fn begin(&mut self, id: &str) -> Result<(), TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !matches!(tx.state, TransactionState::Approved) { return Err(TransactionError::ApprovalRequired); }
        tx.state = TransactionState::Executing;
        Ok(())
    }

    pub fn begin_verify(&mut self, id: &str) -> Result<(), TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !matches!(tx.state, TransactionState::Executing) { return Err(TransactionError::InvalidState); }
        tx.state = TransactionState::Verifying;
        Ok(())
    }

    pub fn commit(&mut self, id: &str) -> Result<(), TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !matches!(tx.state, TransactionState::Verifying) { return Err(TransactionError::InvalidState); }
        tx.state = TransactionState::Committed;
        Ok(())
    }

    pub fn rollback(&mut self, id: &str) -> Result<(), TransactionError> {
        let tx = self.transactions.get_mut(id).ok_or_else(|| TransactionError::NotFound(id.into()))?;
        if !tx.actions.iter().all(|a| a.reversible) { return Err(TransactionError::NotReversible); }
        tx.state = TransactionState::RollingBack;
        tx.state = TransactionState::RolledBack;
        Ok(())
    }
}

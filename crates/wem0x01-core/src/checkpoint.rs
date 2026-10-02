use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Checkpoint {
    pub id: String,
    pub transaction_id: String,
    pub state_revision: u64,
    pub state_digest: String,
    pub resources: BTreeMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CheckpointError {
    #[error("checkpoint id or transaction id is empty")]
    InvalidIdentity,
    #[error("resource key is empty")]
    InvalidResourceKey,
}

impl Checkpoint {
    pub fn new(
        id: impl Into<String>,
        transaction_id: impl Into<String>,
        state_revision: u64,
        state_digest: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            transaction_id: transaction_id.into(),
            state_revision,
            state_digest: state_digest.into(),
            resources: BTreeMap::new(),
        }
    }

    pub fn record_resource(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), CheckpointError> {
        let key = key.into();
        if key.is_empty() {
            return Err(CheckpointError::InvalidResourceKey);
        }
        self.resources.insert(key, value.into());
        Ok(())
    }

    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.id.is_empty() || self.transaction_id.is_empty() {
            return Err(CheckpointError::InvalidIdentity);
        }
        if self.resources.keys().any(String::is_empty) {
            return Err(CheckpointError::InvalidResourceKey);
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct CheckpointStore {
    checkpoints: BTreeMap<String, Checkpoint>,
}

impl CheckpointStore {
    pub fn insert(&mut self, checkpoint: Checkpoint) -> Result<(), CheckpointError> {
        checkpoint.validate()?;
        self.checkpoints.insert(checkpoint.id.clone(), checkpoint);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Checkpoint> {
        self.checkpoints.get(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<Checkpoint> {
        self.checkpoints.remove(id)
    }

    pub fn find_by_transaction(&self, transaction_id: &str) -> Option<&Checkpoint> {
        self.checkpoints
            .values()
            .find(|cp| cp.transaction_id == transaction_id)
    }

    pub fn len(&self) -> usize {
        self.checkpoints.len()
    }

    pub fn is_empty(&self) -> bool {
        self.checkpoints.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_records_state_and_resources() {
        let mut checkpoint = Checkpoint::new("cp-1", "tx-1", 42, "sha256:test");
        checkpoint
            .record_resource("compositor.workspace", "3")
            .unwrap();

        let mut store = CheckpointStore::default();
        store.insert(checkpoint).unwrap();

        let saved = store.get("cp-1").unwrap();
        assert_eq!(saved.state_revision, 42);
        assert_eq!(saved.resources["compositor.workspace"], "3");
    }

    #[test]
    fn empty_identity_is_rejected() {
        let checkpoint = Checkpoint::new("", "tx-1", 1, "digest");
        assert!(matches!(
            checkpoint.validate(),
            Err(CheckpointError::InvalidIdentity)
        ));
    }
}

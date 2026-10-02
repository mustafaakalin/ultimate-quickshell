use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationEffect {
    Read,
    Control,
    Mutate,
    Privileged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Idempotency {
    Idempotent,
    NonIdempotent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationSpec {
    pub id: String,
    pub version: u16,
    pub capability: String,
    pub effect: OperationEffect,
    pub input_schema: String,
    pub output_schema: String,
    pub timeout_ms: u64,
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub idempotency: Idempotency,
    pub reversible: bool,
    pub rollback_operation: Option<String>,
    pub verification_operation: Option<String>,
}

impl OperationSpec {
    pub fn validate(&self) -> Result<(), OperationSpecError> {
        if self.id.is_empty() || self.id.len() > 256 {
            return Err(OperationSpecError::InvalidId);
        }
        if self.capability.is_empty() {
            return Err(OperationSpecError::MissingCapability);
        }
        if self.version == 0 || self.timeout_ms == 0 {
            return Err(OperationSpecError::InvalidVersionOrTimeout);
        }
        if self.max_input_bytes == 0 || self.max_output_bytes == 0 {
            return Err(OperationSpecError::InvalidBounds);
        }
        if self.reversible && self.rollback_operation.is_none() {
            return Err(OperationSpecError::MissingRollback);
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OperationSpecError {
    #[error("invalid operation id")]
    InvalidId,
    #[error("missing capability")]
    MissingCapability,
    #[error("invalid version or timeout")]
    InvalidVersionOrTimeout,
    #[error("invalid input/output bounds")]
    InvalidBounds,
    #[error("reversible operation must declare rollback")]
    MissingRollback,
}

#[derive(Debug, Default)]
pub struct OperationRegistry {
    operations: std::collections::BTreeMap<String, OperationSpec>,
}

impl OperationRegistry {
    pub fn register(&mut self, operation: OperationSpec) -> Result<(), OperationSpecError> {
        operation.validate()?;
        self.operations.insert(operation.id.clone(), operation);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&OperationSpec> {
        self.operations.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &OperationSpec> {
        self.operations.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> OperationSpec {
        OperationSpec {
            id: "pipewire.sink.set_default".into(),
            version: 1,
            capability: "audio.control".into(),
            effect: OperationEffect::Control,
            input_schema: "sink-id".into(),
            output_schema: "ack".into(),
            timeout_ms: 1_000,
            max_input_bytes: 4096,
            max_output_bytes: 8192,
            idempotency: Idempotency::Idempotent,
            reversible: true,
            rollback_operation: Some("pipewire.sink.restore_default".into()),
            verification_operation: Some("pipewire.sink.get_default".into()),
        }
    }

    #[test]
    fn rejects_reversible_operation_without_rollback() {
        let mut operation = spec();
        operation.rollback_operation = None;
        assert!(matches!(
            operation.validate(),
            Err(OperationSpecError::MissingRollback)
        ));
    }

    #[test]
    fn registers_valid_operation() {
        let mut registry = OperationRegistry::default();
        registry.register(spec()).unwrap();
        assert!(registry.get("pipewire.sink.set_default").is_some());
    }
}

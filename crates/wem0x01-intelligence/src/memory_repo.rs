use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::{MemoryEntry, MemoryKind, MemoryTrust, IntelligenceError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub entry: MemoryEntry,
    pub encrypted: bool,
    pub key_version: u32,
}

#[derive(Debug, Default)]
pub struct MemoryRepository {
    namespaces: BTreeMap<String, Vec<MemoryRecord>>,
}

impl MemoryRepository {
    pub fn insert(&mut self, entry: MemoryEntry, encrypted: bool, key_version: u32) -> Result<(), IntelligenceError> {
        if entry.namespace.is_empty() || entry.namespace.len() > 128 {
            return Err(IntelligenceError::Memory("invalid namespace".into()));
        }
        self.namespaces.entry(entry.namespace.clone()).or_default()
            .push(MemoryRecord { entry, encrypted, key_version });
        Ok(())
    }

    pub fn recent(&self, namespace: &str, limit: usize) -> Vec<&MemoryRecord> {
        self.namespaces.get(namespace)
            .map(|v| v.iter().rev().take(limit).collect())
            .unwrap_or_default()
    }

    pub fn purge_namespace(&mut self, namespace: &str) {
        self.namespaces.remove(namespace);
    }
}

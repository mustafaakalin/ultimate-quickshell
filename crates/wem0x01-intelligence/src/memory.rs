use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::IntelligenceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKind { Working, Episodic, Semantic, Procedural }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTrust { Untrusted, Observed, Verified, UserProvided }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub kind: MemoryKind,
    pub trust: MemoryTrust,
    pub namespace: String,
    pub content: String,
    pub source: Option<String>,
    pub created_unix_ms: u64,
    pub expires_unix_ms: Option<u64>,
    pub sensitivity: u8,
}

pub trait MemoryStore: Send + Sync {
    fn put(&mut self, entry: MemoryEntry) -> Result<(), IntelligenceError>;
    fn recent(&self, namespace: &str, limit: usize) -> Vec<MemoryEntry>;
}

#[derive(Debug)]
pub struct BoundedMemoryStore {
    capacity: usize,
    entries: VecDeque<MemoryEntry>,
}

impl BoundedMemoryStore {
    pub fn new(capacity: usize) -> Self {
        Self { capacity: capacity.max(1), entries: VecDeque::new() }
    }
}

impl MemoryStore for BoundedMemoryStore {
    fn put(&mut self, entry: MemoryEntry) -> Result<(), IntelligenceError> {
        if self.entries.len() >= self.capacity { self.entries.pop_front(); }
        self.entries.push_back(entry);
        Ok(())
    }

    fn recent(&self, namespace: &str, limit: usize) -> Vec<MemoryEntry> {
        self.entries.iter().rev()
            .filter(|entry| entry.namespace == namespace)
            .take(limit)
            .cloned()
            .collect()
    }
}

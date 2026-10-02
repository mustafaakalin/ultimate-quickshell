use std::collections::VecDeque;
use tokio::sync::RwLock;
use wem0x01_protocol::Incident;

#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub sequence: u64,
    pub incident: Incident,
}

pub struct IncidentJournal {
    capacity: usize,
    next_sequence: RwLock<u64>,
    entries: RwLock<VecDeque<JournalEntry>>,
}

impl IncidentJournal {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "incident journal capacity must be > 0");
        Self {
            capacity,
            next_sequence: RwLock::new(0),
            entries: RwLock::new(VecDeque::with_capacity(capacity)),
        }
    }

    pub async fn append(&self, incident: Incident) -> u64 {
        let mut sequence = self.next_sequence.write().await;
        *sequence = sequence.saturating_add(1);
        let entry = JournalEntry { sequence: *sequence, incident };

        let mut entries = self.entries.write().await;
        if entries.len() == self.capacity {
            entries.pop_front();
        }
        entries.push_back(entry);
        *sequence
    }

    pub async fn recent(&self, limit: usize) -> Vec<JournalEntry> {
        let entries = self.entries.read().await;
        entries.iter().rev().take(limit).cloned().collect()
    }

    pub async fn since(&self, sequence: u64, limit: usize) -> Vec<JournalEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .filter(|entry| entry.sequence > sequence)
            .take(limit)
            .cloned()
            .collect()
    }
}

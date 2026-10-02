//! Security-first event/state core.

use async_trait::async_trait;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{RwLock, broadcast};

pub mod actor;
pub mod checkpoint;
pub mod journal;
pub mod plugin;
pub mod reducer;
pub mod transaction;
pub use actor::{Actor, ActorContext, RestartPolicy, Supervisor};
pub use checkpoint::{Checkpoint, CheckpointError, CheckpointStore};
pub use journal::{IncidentJournal, JournalEntry};
pub use plugin::{
    BuiltInPlugin, PluginContext, PluginError, PluginKind, PluginManifest, PluginRecord,
    PluginRegistry,
};
pub use reducer::StateReducer;
pub use transaction::{
    Postcondition, Precondition, Transaction, TransactionAction, TransactionEngine,
    TransactionError, TransactionState,
};
use wem0x01_protocol::{
    AgentIntent, Capability, Command, CompositorEvent, EnvironmentSnapshot, Incident,
    TransactionPlan,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentState {
    pub generation: u64,
    pub compositor: String,
    pub session_id: String,
    pub profile: String,
    pub capabilities: Vec<Capability>,
}

impl EnvironmentState {
    pub fn digest(&self) -> Result<String, serde_json::Error> {
        let encoded = serde_json::to_vec(self)?;
        let digest = Sha256::digest(encoded);
        Ok(format!("{digest:x}"))
    }
}

#[derive(Debug, Clone)]
pub enum EnvironmentEvent {
    Snapshot(Arc<EnvironmentState>),
    Command(Command),
    Compositor(CompositorEvent),
    ComponentChanged { id: String },
    Incident(Incident),
    AgentIntent(AgentIntent),
    TransactionPlanned(TransactionPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Read,
    Control,
    SpawnProcess,
    WriteConfig,
    Privileged,
}

#[derive(Debug, Clone)]
pub struct Principal {
    pub id: String,
    pub effects: HashSet<Effect>,
}

impl Principal {
    pub fn frontend(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            effects: HashSet::from([Effect::Read, Effect::Control]),
        }
    }

    pub fn plugin(id: impl Into<String>, effects: impl IntoIterator<Item = Effect>) -> Self {
        Self {
            id: id.into(),
            effects: effects.into_iter().collect(),
        }
    }

    pub fn agent(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            effects: HashSet::from([Effect::Read]),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("effect {effect:?} is not granted to {principal}")]
    Denied { principal: String, effect: Effect },
    #[error("unknown capability: {capability}")]
    UnknownCapability { capability: String },
}

#[derive(Debug, Clone)]
pub struct CapabilityDescriptor {
    pub id: String,
    pub effect: Effect,
}

#[derive(Default)]
pub struct CapabilityRegistry {
    descriptors: std::collections::HashMap<String, CapabilityDescriptor>,
}

impl CapabilityRegistry {
    pub fn register(&mut self, id: impl Into<String>, effect: Effect) {
        let id = id.into();
        self.descriptors
            .insert(id.clone(), CapabilityDescriptor { id, effect });
    }

    pub fn effect_for(&self, id: &str) -> Option<Effect> {
        self.descriptors.get(id).map(|descriptor| descriptor.effect)
    }

    pub fn authorize(
        &self,
        policy: &PolicyEngine,
        principal: &Principal,
        capability: &str,
    ) -> Result<(), PolicyError> {
        let effect = self
            .effect_for(capability)
            .ok_or_else(|| PolicyError::UnknownCapability {
                capability: capability.to_owned(),
            })?;
        policy.authorize(principal, effect)
    }
}

#[derive(Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    pub fn authorize(&self, principal: &Principal, effect: Effect) -> Result<(), PolicyError> {
        if principal.effects.contains(&effect) {
            Ok(())
        } else {
            Err(PolicyError::Denied {
                principal: principal.id.clone(),
                effect,
            })
        }
    }
}

pub struct StateStore {
    state: RwLock<Arc<EnvironmentState>>,
}

impl StateStore {
    pub fn new(initial: EnvironmentState) -> Self {
        Self {
            state: RwLock::new(Arc::new(initial)),
        }
    }

    pub async fn snapshot(&self) -> Arc<EnvironmentState> {
        self.state.read().await.clone()
    }

    pub async fn replace(&self, mut next: EnvironmentState) -> Arc<EnvironmentState> {
        let current = self.snapshot().await;
        next.generation = current.generation.saturating_add(1);
        let next = Arc::new(next);
        *self.state.write().await = next.clone();
        next
    }
}

pub struct EventBus {
    tx: broadcast::Sender<EnvironmentEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn publish(&self, event: EnvironmentEvent) {
        let _ = self.tx.send(event);
    }
    pub fn subscribe(&self) -> broadcast::Receiver<EnvironmentEvent> {
        self.tx.subscribe()
    }
}

#[async_trait]
pub trait EnvironmentProvider: Send + Sync {
    async fn snapshot(&self) -> anyhow::Result<EnvironmentSnapshot>;
    async fn execute(&self, command: Command) -> anyhow::Result<()>;
}

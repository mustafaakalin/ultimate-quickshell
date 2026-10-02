//! Security-first event/state core.
//!
//! The core owns system truth. Frontends are observers and command clients.
//! State snapshots are immutable values; mutations happen only through typed
//! commands accepted by the policy boundary.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use wem0x01_protocol::{Capability, Command, CompositorEvent, EnvironmentSnapshot};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentState {
    pub generation: u64,
    pub compositor: String,
    pub session_id: String,
    pub profile: String,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone)]
pub enum EnvironmentEvent {
    Snapshot(Arc<EnvironmentState>),
    Command(Command),
    Compositor(CompositorEvent),
    ComponentChanged { id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        Self { id: id.into(), effects: HashSet::from([Effect::Read, Effect::Control]) }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("effect {effect:?} is not granted to {principal}")]
    Denied { principal: String, effect: Effect },
}

#[derive(Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    pub fn authorize(&self, principal: &Principal, effect: Effect) -> Result<(), PolicyError> {
        if principal.effects.contains(&effect) {
            Ok(())
        } else {
            Err(PolicyError::Denied { principal: principal.id.clone(), effect })
        }
    }
}

pub struct StateStore {
    state: RwLock<Arc<EnvironmentState>>,
}

impl StateStore {
    pub fn new(initial: EnvironmentState) -> Self {
        Self { state: RwLock::new(Arc::new(initial)) }
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

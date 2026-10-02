//! Event-driven environment orchestration primitives.
//!
//! Core deliberately knows nothing about Hyprland, Sway, Niri, Quickshell,
//! GTK or a particular service manager. Platform adapters implement those edges.

use async_trait::async_trait;
use lem0x01_protocol::{Command, CompositorEvent, EnvironmentSnapshot};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum EnvironmentEvent {
    Snapshot(EnvironmentSnapshot),
    Command(Command),
    Compositor(CompositorEvent),
    ComponentChanged { id: String },
}

#[async_trait]
pub trait EnvironmentProvider: Send + Sync {
    async fn snapshot(&self) -> anyhow::Result<EnvironmentSnapshot>;
    async fn execute(&self, command: Command) -> anyhow::Result<()>;
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

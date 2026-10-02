//! Stable control-plane types shared by the daemon, CLI and UI adapters.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSnapshot {
    pub compositor: String,
    pub session_id: String,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Reload,
    SetProfile(String),
    OpenSurface(String),
    CloseSurface(String),
    RestartComponent(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorEvent {
    pub compositor: String,
    pub name: String,
    pub data: String,
}

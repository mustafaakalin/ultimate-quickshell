//! Stable wire types. Keep this crate small so every frontend can consume it.

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSnapshot {
    pub protocol: u16,
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

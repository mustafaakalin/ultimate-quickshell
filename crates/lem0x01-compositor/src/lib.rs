//! Compositor-neutral adapter layer.
//!
//! The core discovers capabilities instead of assuming one compositor.
//! Adapters may use native IPC and Wayland protocols; the rest of lem0x01
//! consumes normalized events.

use lem0x01_protocol::{Capability, EnvironmentSnapshot};

pub trait CompositorAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    fn capabilities(&self) -> Vec<Capability>;
    fn snapshot(&self) -> anyhow::Result<EnvironmentSnapshot>;
}

pub mod detection {
    pub fn compositor_hint() -> String {
        std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_else(|_| "unknown".into())
    }
}

pub mod hyprland;

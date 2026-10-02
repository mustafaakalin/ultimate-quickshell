//! Platform integration primitives for wem0x01.
//!
//! This crate is intentionally small at the workspace-bootstrap stage.
//! Concrete compositor, D-Bus, PipeWire, NetworkManager and systemd
//! integrations will be added behind typed adapter boundaries.

pub const PLATFORM_API_VERSION: u16 = 1;

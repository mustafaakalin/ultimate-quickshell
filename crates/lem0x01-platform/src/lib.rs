//! Linux platform boundary.
//!
//! This crate owns integration with user-session infrastructure (D-Bus,
//! systemd --user, portals, PipeWire/BlueZ/NetworkManager adapters, etc.).
//! No UI code belongs here.

pub mod session {
    #[derive(Debug, Clone, Copy)]
    pub struct UserSession;

    impl UserSession {
        pub fn current() -> Self { Self }
    }
}

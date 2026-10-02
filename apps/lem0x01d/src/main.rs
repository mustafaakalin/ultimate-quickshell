use lem0x01_compositor::detection::compositor_hint;
use lem0x01_core::EventBus;
use lem0x01_protocol::{Capability, EnvironmentSnapshot};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "lem0x01d=info".into()),
        )
        .init();

    let bus = EventBus::new(256);
    let snapshot = EnvironmentSnapshot {
        compositor: compositor_hint(),
        session_id: std::env::var("XDG_SESSION_ID").unwrap_or_else(|_| "unknown".into()),
        capabilities: vec![
            Capability { id: "environment.snapshot".into(), available: true },
            Capability { id: "compositor.adapter".into(), available: true },
        ],
    };

    bus.publish(lem0x01_core::EnvironmentEvent::Snapshot(snapshot.clone()));
    info!(compositor = %snapshot.compositor, "lem0x01 control plane started");

    tokio::signal::ctrl_c().await?;
    Ok(())
}

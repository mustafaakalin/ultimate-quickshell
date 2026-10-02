use lem0x01_compositor::detection::compositor_hint;
use lem0x01_core::{EnvironmentEvent, EventBus};
use lem0x01_protocol::{Capability, EnvironmentSnapshot};
use tracing::{error, info};

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

    bus.publish(EnvironmentEvent::Snapshot(snapshot.clone()));
    info!(compositor = %snapshot.compositor, "lem0x01 control plane started");

    if snapshot.compositor.to_ascii_lowercase().contains("hypr") {
        let events = lem0x01_compositor::hyprland::HyprlandEvents::from_environment();
        if let Ok(events) = events {
            let tx = bus;
            tokio::spawn(async move {
                if let Err(error) = events.run(|event| tx.publish(EnvironmentEvent::Compositor(event))).await {
                    error!(%error, "Hyprland event stream stopped");
                }
            });
        }
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}

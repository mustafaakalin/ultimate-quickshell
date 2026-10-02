use std::sync::Arc;
use tracing::{error, info};
use wem0x01_compositor::detection::compositor_hint;
use wem0x01_core::{EnvironmentEvent, EnvironmentState, EventBus, PolicyEngine, Principal, StateStore};
use wem0x01_protocol::Capability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "wem0x01d=info".into()))
        .init();

    let bus = EventBus::new(512);
    let policy = PolicyEngine::default();
    let frontend = Principal::frontend("local-ui");
    policy.authorize(&frontend, wem0x01_core::Effect::Read)?;

    let initial = EnvironmentState {
        generation: 0,
        compositor: compositor_hint(),
        session_id: std::env::var("XDG_SESSION_ID").unwrap_or_else(|_| "unknown".into()),
        profile: "default".into(),
        capabilities: vec![
            Capability { id: "environment.snapshot".into(), available: true },
            Capability { id: "compositor.adapter".into(), available: true },
        ],
    };

    let state = Arc::new(StateStore::new(initial));
    let snapshot = state.snapshot().await;
    bus.publish(EnvironmentEvent::Snapshot(snapshot.clone()));
    info!(compositor = %snapshot.compositor, generation = snapshot.generation, "wem0x01 control plane started");

    if snapshot.compositor.to_ascii_lowercase().contains("hypr") {
        if let Ok(events) = wem0x01_compositor::hyprland::HyprlandEvents::from_environment() {
            let tx = bus;
            tokio::spawn(async move {
                if let Err(error) = events.run(|event| tx.publish(EnvironmentEvent::Compositor(event))).await {
                    error!(%error, "Hyprland event actor stopped");
                }
            });
        }
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}

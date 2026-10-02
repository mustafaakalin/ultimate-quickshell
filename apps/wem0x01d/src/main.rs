use std::sync::Arc;
use tracing::{error, info};
use wem0x01_compositor::detection::compositor_hint;
use wem0x01_core::{
    CapabilityRegistry, Effect, EnvironmentEvent, EnvironmentState, EventBus, PolicyEngine,
    Principal, StateStore,
};
use wem0x01_protocol::Capability;

mod ipc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "wem0x01d=info".into()))
        .init();

    let bus = EventBus::new(512);
    let policy = Arc::new(PolicyEngine);
    let frontend = Principal::frontend("local-ui");
    let mut capabilities = CapabilityRegistry::default();
    capabilities.register("environment.snapshot", Effect::Read);
    capabilities.register("environment.control", Effect::Control);
    capabilities.register("process.spawn", Effect::SpawnProcess);
    capabilities.register("config.write", Effect::WriteConfig);
    capabilities.register("privileged.operation", Effect::Privileged);
    capabilities.authorize(&policy, &frontend, "environment.snapshot")?;
    let capabilities = Arc::new(capabilities);

    let initial = EnvironmentState {
        generation: 0,
        compositor: compositor_hint(),
        session_id: std::env::var("XDG_SESSION_ID").unwrap_or_else(|_| "unknown".into()),
        profile: "default".into(),
        capabilities: vec![
            Capability {
                id: "environment.snapshot".into(),
                available: true,
            },
            Capability {
                id: "compositor.adapter".into(),
                available: true,
            },
        ],
    };

    let state = Arc::new(StateStore::new(initial));
    let snapshot = state.snapshot().await;
    bus.publish(EnvironmentEvent::Snapshot(snapshot.clone()));
    info!(compositor = %snapshot.compositor, generation = snapshot.generation, "wem0x01 control plane started");

    if snapshot.compositor.to_ascii_lowercase().contains("hypr")
        && let Ok(events) = wem0x01_compositor::hyprland::HyprlandEvents::from_environment() {
            let tx = bus;
            tokio::spawn(async move {
                if let Err(error) = events
                    .run(|event| tx.publish(EnvironmentEvent::Compositor(event)))
                    .await
                {
                    error!(%error, "Hyprland event actor stopped");
                }
            });
        }

    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("XDG_RUNTIME_DIR is required for local IPC"))?;
    let socket = runtime_dir.join("wem0x01.sock");

    let server = ipc::IpcServer::new(
        socket.clone(),
        Arc::clone(&state),
        Arc::clone(&policy),
        Arc::clone(&capabilities),
    );
    let mut ipc_task = tokio::spawn(async move { server.run().await });

    tokio::select! {
        signal = tokio::signal::ctrl_c() => {
            signal?;
            ipc_task.abort();
        }
        result = &mut ipc_task => {
            result??;
        }
    }

    let _ = tokio::fs::remove_file(socket).await;
    Ok(())
}

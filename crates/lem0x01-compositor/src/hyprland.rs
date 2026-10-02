use anyhow::{Context, Result};
use lem0x01_protocol::CompositorEvent;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
};
use tracing::{debug, warn};

/// Native Hyprland socket2 event stream.
///
/// Hyprland publishes `EVENT>>DATA\n` on the event socket. We consume that
/// stream directly instead of polling hyprctl, keeping the control plane
/// responsive and avoiding repeated synchronous IPC calls.
pub struct HyprlandEvents {
    path: std::path::PathBuf,
}

impl HyprlandEvents {
    pub fn from_environment() -> Result<Self> {
        let runtime = std::env::var_os("XDG_RUNTIME_DIR")
            .context("XDG_RUNTIME_DIR is not set")?;
        let signature = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
            .context("HYPRLAND_INSTANCE_SIGNATURE is not set")?;

        let path = std::path::PathBuf::from(runtime)
            .join("hypr")
            .join(signature)
            .join(".socket2.sock");

        Ok(Self { path })
    }

    pub async fn run<F>(&self, mut on_event: F) -> Result<()>
    where
        F: FnMut(CompositorEvent) + Send,
    {
        let stream = UnixStream::connect(&self.path)
            .await
            .with_context(|| format!("connect to {}", self.path.display()))?;

        let mut lines = BufReader::new(stream).lines();

        while let Some(line) = lines.next_line().await? {
            let Some((name, data)) = line.split_once(">>") else {
                warn!(line = %line, "ignored malformed Hyprland event");
                continue;
            };

            let event = CompositorEvent {
                compositor: "hyprland".into(),
                name: name.to_owned(),
                data: data.to_owned(),
            };

            debug!(event = %event.name, "hyprland event");
            on_event(event);
        }

        Ok(())
    }
}

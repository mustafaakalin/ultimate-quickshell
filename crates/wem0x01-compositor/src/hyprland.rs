use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
};
use tracing::warn;
use wem0x01_protocol::CompositorEvent;

pub struct HyprlandEvents {
    path: std::path::PathBuf,
}

impl HyprlandEvents {
    pub fn from_environment() -> Result<Self> {
        let runtime = std::env::var_os("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is not set")?;
        let signature = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
            .context("HYPRLAND_INSTANCE_SIGNATURE is not set")?;
        Ok(Self {
            path: std::path::PathBuf::from(runtime)
                .join("hypr")
                .join(signature)
                .join(".socket2.sock"),
        })
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
                warn!(line = %line, "ignored malformed compositor event");
                continue;
            };
            on_event(CompositorEvent {
                compositor: "hyprland".into(),
                name: name.into(),
                data: data.into(),
            });
        }
        Ok(())
    }
}

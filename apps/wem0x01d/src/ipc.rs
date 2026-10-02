#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{UnixListener, UnixStream},
};
use tracing::{info, warn};
use wem0x01_core::{CapabilityRegistry, PolicyEngine, Principal, StateStore};
use wem0x01_protocol::{
    ClientKind, Command, EnvironmentSnapshot, IPC_MAX_FRAME, IpcHello, IpcRequest, IpcResponse,
    PROTOCOL_VERSION,
};

pub struct IpcServer {
    path: PathBuf,
    state: Arc<StateStore>,
    policy: Arc<PolicyEngine>,
    capabilities: Arc<CapabilityRegistry>,
}

impl IpcServer {
    pub fn new(
        path: impl Into<PathBuf>,
        state: Arc<StateStore>,
        policy: Arc<PolicyEngine>,
        capabilities: Arc<CapabilityRegistry>,
    ) -> Self {
        Self {
            path: path.into(),
            state,
            policy,
            capabilities,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        if Path::new(&self.path).exists() {
            tokio::fs::remove_file(&self.path).await?;
        }

        let listener = UnixListener::bind(&self.path)?;
        set_socket_mode(&self.path, 0o600)?;

        info!(socket = %self.path.display(), "IPC server listening");

        loop {
            let (stream, _) = listener.accept().await?;
            let state = Arc::clone(&self.state);
            let policy = Arc::clone(&self.policy);
            let capabilities = Arc::clone(&self.capabilities);

            tokio::spawn(async move {
                if let Err(error) = handle_client(stream, state, policy, capabilities).await {
                    warn!(%error, "IPC client disconnected with error");
                }
            });
        }
    }
}

async fn handle_client(
    stream: UnixStream,
    state: Arc<StateStore>,
    policy: Arc<PolicyEngine>,
    capabilities: Arc<CapabilityRegistry>,
) -> anyhow::Result<()> {
    let peer_uid = peer_uid(&stream)?;
    let local_uid = std::fs::metadata("/proc/self")?.uid();
    if peer_uid != local_uid {
        anyhow::bail!("IPC peer UID {peer_uid} is not the daemon UID {local_uid}");
    }

    let mut reader = BufReader::new(stream);
    let mut line = String::with_capacity(1024);

    reader.read_line(&mut line).await?;
    if line.len() > IPC_MAX_FRAME {
        anyhow::bail!("IPC hello exceeds maximum frame size");
    }

    let hello_request: IpcRequest = serde_json::from_str(&line)?;
    let hello = hello_request
        .hello
        .ok_or_else(|| anyhow::anyhow!("missing IPC hello"))?;

    authenticate_hello(&hello, &policy, &capabilities)?;

    let snapshot = state.snapshot().await;
    let response = IpcResponse {
        id: hello_request.id,
        ok: true,
        error: None,
        snapshot: Some(snapshot_to_protocol(&snapshot)),
    };

    let mut stream = reader.into_inner();
    write_response(&mut stream, &response).await?;

    loop {
        line.clear();
        let read = {
            let mut reader = BufReader::new(&mut stream);
            reader.read_line(&mut line).await?
        };
        if read == 0 {
            break;
        }
        if line.len() > IPC_MAX_FRAME {
            anyhow::bail!("IPC frame exceeds maximum size");
        }

        let request: IpcRequest = serde_json::from_str(&line)?;
        let response = match request.command {
            Some(Command::Reload) => IpcResponse {
                id: request.id,
                ok: true,
                error: None,
                snapshot: Some(snapshot_to_protocol(&state.snapshot().await)),
            },
            Some(Command::Agent(_)) => IpcResponse {
                id: request.id,
                ok: false,
                error: Some("agent commands require the transaction broker".into()),
                snapshot: None,
            },
            Some(_) => IpcResponse {
                id: request.id,
                ok: false,
                error: Some("command execution is not enabled in this core milestone".into()),
                snapshot: None,
            },
            None => IpcResponse {
                id: request.id,
                ok: false,
                error: Some("missing command".into()),
                snapshot: None,
            },
        };
        write_response(&mut stream, &response).await?;
    }

    Ok(())
}

fn authenticate_hello(
    hello: &IpcHello,
    policy: &PolicyEngine,
    capabilities: &CapabilityRegistry,
) -> anyhow::Result<()> {
    if hello.protocol != PROTOCOL_VERSION {
        anyhow::bail!(
            "unsupported IPC protocol {}; expected {}",
            hello.protocol,
            PROTOCOL_VERSION
        );
    }

    let principal = match hello.kind.unwrap_or(ClientKind::Cli) {
        ClientKind::Ui | ClientKind::Cli => Principal::frontend(hello.client.clone()),
        ClientKind::Agent => Principal::agent(hello.client.clone()),
        ClientKind::Plugin => Principal::plugin(hello.client.clone(), [wem0x01_core::Effect::Read]),
        ClientKind::Automation => {
            Principal::plugin(hello.client.clone(), [wem0x01_core::Effect::Read])
        }
    };
    for capability in &hello.requested_capabilities {
        capabilities.authorize(policy, &principal, capability)?;
    }
    Ok(())
}

async fn write_response(stream: &mut UnixStream, response: &IpcResponse) -> anyhow::Result<()> {
    let mut encoded = serde_json::to_vec(response)?;
    encoded.push(b'\n');
    if encoded.len() > IPC_MAX_FRAME {
        anyhow::bail!("IPC response exceeds maximum frame size");
    }
    stream.write_all(&encoded).await?;
    Ok(())
}

fn snapshot_to_protocol(state: &wem0x01_core::EnvironmentState) -> EnvironmentSnapshot {
    EnvironmentSnapshot {
        protocol: PROTOCOL_VERSION,
        compositor: state.compositor.clone(),
        session_id: state.session_id.clone(),
        capabilities: state.capabilities.clone(),
    }
}

#[cfg(unix)]
fn peer_uid(stream: &UnixStream) -> anyhow::Result<u32> {
    Ok(stream.peer_cred()?.uid())
}

fn set_socket_mode(path: &Path, mode: u32) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = std::fs::metadata(path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}

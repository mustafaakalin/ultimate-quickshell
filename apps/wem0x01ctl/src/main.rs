use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::UnixStream};
use wem0x01_protocol::{ClientKind, IpcHello, IpcRequest, IpcResponse, IPC_MAX_FRAME, PROTOCOL_VERSION};
use wem0x01_protocol::Command;

#[derive(Parser)]
#[command(name = "wem0x01ctl", about = "Control the wem0x01 Wayland environment")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Reload,
    Profile { name: String },
    Surface { action: String, name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = match Cli::parse().command {
        Commands::Reload => Command::Reload,
        Commands::Profile { name } => Command::SetProfile(name),
        Commands::Surface { action, name } => {
            if action == "open" { Command::OpenSurface(name) }
            else { Command::CloseSurface(name) }
        }
    };
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).ok_or_else(|| anyhow::anyhow!("XDG_RUNTIME_DIR is required"))?;
    let stream = UnixStream::connect(runtime_dir.join("wem0x01.sock")).await?;
    let mut io = BufReader::new(stream);
    send(&mut io, IpcRequest { id: 1, hello: Some(IpcHello { protocol: PROTOCOL_VERSION, client: "wem0x01ctl".into(), kind: Some(ClientKind::Cli), requested_capabilities: vec!["environment.snapshot".into(), "environment.control".into()] }), command: None }).await?;
    let handshake = read_response(&mut io).await?;
    if !handshake.ok { anyhow::bail!(handshake.error.unwrap_or_else(|| "IPC handshake failed".into())); }
    send(&mut io, IpcRequest { id: 2, hello: None, command: Some(command) }).await?;
    let response = read_response(&mut io).await?;
    if !response.ok { anyhow::bail!(response.error.unwrap_or_else(|| "IPC command failed".into())); }
    if let Some(snapshot) = response.snapshot { println!("{}", serde_json::to_string_pretty(&snapshot)?); }
    Ok(())
}

async fn send(io: &mut BufReader<UnixStream>, request: IpcRequest) -> anyhow::Result<()> {
    let mut bytes = serde_json::to_vec(&request)?;
    bytes.push(b'\n');
    if bytes.len() > IPC_MAX_FRAME { anyhow::bail!("IPC request exceeds maximum frame size"); }
    io.get_mut().write_all(&bytes).await?;
    Ok(())
}

async fn read_response(io: &mut BufReader<UnixStream>) -> anyhow::Result<IpcResponse> {
    let mut line = String::with_capacity(1024);
    let read = io.read_line(&mut line).await?;
    if read == 0 { anyhow::bail!("IPC server closed the connection"); }
    if line.len() > IPC_MAX_FRAME { anyhow::bail!("IPC response exceeds maximum frame size"); }
    Ok(serde_json::from_str(&line)?)
}

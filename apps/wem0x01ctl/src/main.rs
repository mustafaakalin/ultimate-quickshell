use clap::{Parser, Subcommand};
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

fn main() {
    let command = match Cli::parse().command {
        Commands::Reload => Command::Reload,
        Commands::Profile { name } => Command::SetProfile(name),
        Commands::Surface { action, name } => {
            if action == "open" { Command::OpenSurface(name) }
            else { Command::CloseSurface(name) }
        }
    };
    println!("{}", serde_json::to_string(&command).expect("protocol serialization"));
}

use clap::{Parser, Subcommand};
use lem0x01_protocol::Command;

#[derive(Parser)]
#[command(name = "lem0x01ctl", about = "Control the lem0x01 Linux environment")]
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
    let cli = Cli::parse();
    let command = match cli.command {
        Commands::Reload => Command::Reload,
        Commands::Profile { name } => Command::SetProfile(name),
        Commands::Surface { action, name } => {
            if action == "open" {
                Command::OpenSurface(name)
            } else {
                Command::CloseSurface(name)
            }
        }
    };

    println!("{}", serde_json::to_string(&command).expect("protocol serialization"));
}

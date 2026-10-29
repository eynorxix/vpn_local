use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vpnlocal")]
#[command(about = "vpnlocal CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Activate,
    Connect,
    Disconnect,
    Stop,
    Status,
}

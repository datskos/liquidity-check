use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Simulate burn
    SimBurn {
        #[arg(short, long)]
        token: u64,

        #[arg(short, long)]
        block: u64,

        #[arg(short, long)]
        rpc: Option<String>,
    },
}

#[derive(Debug, Parser)]
#[clap(name = "app", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

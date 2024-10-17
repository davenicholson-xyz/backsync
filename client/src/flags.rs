use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Flags {
    #[command(subcommand)]
    pub command: Option<Action>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    INIT {
        #[arg(short, long, env)]
        port: Option<i32>,
    },
    WALLPAPER {
        #[arg(short, long)]
        lock: bool,
    },
    STOP,
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

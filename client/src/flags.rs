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
        /// Lock the wallpaper from changes by server
        #[arg(short, long)]
        lock: bool,
        /// Unlock the wallpaper and allow changes by server
        #[arg(short, long)]
        unlock: bool,
    },
    STOP,
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

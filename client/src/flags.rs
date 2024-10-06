use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Flags {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    START {
        #[arg(short, long, env)]
        port: Option<i32>,
    },
    STOP,
    STATUS,
    SET {
        #[arg(short, long)]
        wallpaper: Option<String>,
    },
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

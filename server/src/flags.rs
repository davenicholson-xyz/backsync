use clap::Parser;

#[derive(Parser, Debug)]
pub struct Flags {
    /// Port to run server on
    #[arg(short, long)]
    pub port: Option<i64>,
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

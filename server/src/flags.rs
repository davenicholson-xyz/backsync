use clap::Parser;

#[derive(Parser, Debug)]
pub struct Flags {
    /// Port to run server on
    #[arg(short, long, env)]
    pub port: Option<i64>,
    /// Port to run http server on
    #[arg(short, long, env)]
    pub web_port: Option<i64>,
    /// Localtion of wallpaper folder
    #[arg(short, long, env)]
    pub storage: Option<String>,
}

pub fn cli_args() -> Flags {
    let flags = Flags::parse();
    return flags;
}

mod flags;
mod network;

fn main() {
    let flags = flags::cli_args();
    let port = flags.port.unwrap_or(37878);
}

use simplelog::CombinedLogger;
use simplelog::*;
use std::fs::File;

pub fn start(filename: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(filename).unwrap(),
        ),
    ])
    .unwrap();
}

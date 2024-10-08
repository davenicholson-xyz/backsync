use std::fs::File;

use simplelog::CombinedLogger;
use simplelog::*;

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

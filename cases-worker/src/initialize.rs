use chrono;
use std::fs::File;

use log::info;
use simplelog::*;

fn setup_logging() {
    let datetime = chrono::offset::Local::now().format("%F_%T").to_string();

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
            File::create(format!("log/{}", datetime)).unwrap(),
        ),
    ])
    .unwrap();
}

/// initialize app
pub fn init() {
    setup_logging();
    info!("app initialized")
}

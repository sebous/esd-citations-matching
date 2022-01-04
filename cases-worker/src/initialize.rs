use chrono;
use std::fs;

use log::info;
use simplelog::*;

fn setup_logging() {
    fs::create_dir("log").unwrap();
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
            fs::File::create(format!("log/{}", datetime)).unwrap(),
        ),
    ])
    .unwrap();
}

/// initialize app
pub fn init() {
    setup_logging();
    info!("app initialized")
}

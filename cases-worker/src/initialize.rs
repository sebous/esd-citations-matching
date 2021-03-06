use chrono;
use std::fs;

use log::info;
use simplelog::*;

const LOG_DIR: &str = "log";

fn setup_logging() {
    fs::create_dir_all(LOG_DIR).unwrap();
    let datetime = chrono::offset::Local::now().format("%F_%T").to_string();

    CombinedLogger::init(vec![
        // turn on for log debugging
        TermLogger::new(
            LevelFilter::Off,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            fs::File::create(format!("{LOG_DIR}/{datetime}.log")).unwrap(),
        ),
    ])
    .unwrap();
}

/// initialize app
pub fn init() {
    setup_logging();
    info!("cases-worker initialized")
}

use log::info;
use simplelog::*;

fn setup_logging() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    // TODO: add logging to file (new file w timestamp for each app execution)
}

/// initialize app
pub fn init() {
    setup_logging();
    info!("app initialized")
}

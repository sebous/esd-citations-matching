use log::{info, SetLoggerError};
use simplelog::*;

fn setup_logging() -> Result<(), SetLoggerError> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    // TODO: add logging to file (new file w timestamp for each app execution)

    Ok(())
}

/// initialize app
pub fn init() {
    setup_logging().unwrap();
    info!("app initialized")
}

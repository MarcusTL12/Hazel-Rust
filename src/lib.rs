#[macro_use]
extern crate log;

pub fn init_logging() {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Trace,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Warn,
            simplelog::Config::default(),
            std::fs::File::create("res/logs/warn.log").unwrap(),
        ),
    ])
    .unwrap();
}

pub fn log_test() {
    error!("Error Hazel!!");
    warn!("Warn Hazel!!");
    info!("Info Hazel!!");
    debug!("Debug Hazel!!");
    trace!("Trace Hazel!!");
}

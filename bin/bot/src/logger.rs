use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub fn logger_init() {
    dotenv::dotenv().ok();
    let level = env::var("LOG_LEVEL")
        .unwrap_or("INFO".to_string())
        .parse()
        .unwrap();
    Builder::new()
        .filter_level(LevelFilter::Warn)
        .filter_module("artifacter", level)
        .init();
}

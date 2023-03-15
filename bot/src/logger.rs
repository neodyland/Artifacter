use env_logger::Builder;
use std::env;

pub fn logger_init() {
    Builder::new()
        .filter_level(
            env::var("LOG_LEVEL")
                .unwrap_or("INFO".to_string())
                .parse()
                .unwrap(),
        )
        .init();
}

pub fn enable_debugging() {
    init_logger(log::LevelFilter::Debug)
}

fn init_logger(level: log::LevelFilter) {
    env_logger::Builder::new().filter_level(level).init()
}

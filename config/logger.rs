static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] [{}] {}",
                record.level(),
                record.module_path().unwrap().to_owned() + ".rs",
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .unwrap();
}

#[derive(Debug, Copy, Clone)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            println!("[{:?}] {}", level, message);
        }
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
fn main()
{
    let log = Logger::new(LogLevel::Info);
    log.info("hello,world!");
    log.debug("hello,world!");
    log.error("hello,world!");
    log.warning("hello,world!");
}

use log::{debug, error, info, trace, warn};
use env_logger::Env;

fn main() {
    // Инициализация логгера
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::Builder::from_env(env).init();


    // Примеры записи логов
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}

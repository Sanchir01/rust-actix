use slog::{o, Logger};
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard};
use slog_term::{CompactFormat, TermDecorator};

pub fn init_logger() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    // Устанавливаем глобальный логгер
    let guard = set_global_logger(log);

    // Подключаем стандартное логирование через log::info!(), log::error!() и т.д.
    slog_stdlog::init().unwrap();

    guard
}

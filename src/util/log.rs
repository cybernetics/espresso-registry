use slog::{o, Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

pub fn create_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
extern crate outdated_runner;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::path::Path;
use outdated_runner::Crate;
use slog::{Discard, Drain, Logger};

#[test]
fn check_the_dummy_crate() {
    let krate_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dummy");
    let krate = Crate::from_dir(krate_dir);

    let results = outdated_runner::check_if_outdated(&krate, term_logger()).unwrap();

    assert_eq!(results.len(), 2);
}

#[allow(dead_code)]
fn noop_logger() -> Logger {
    Logger::root(Discard, o!())
}

#[allow(dead_code)]
fn term_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}

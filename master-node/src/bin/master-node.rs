extern crate dotenv;
extern crate failure;
extern crate master_node;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::env;
use std::process;
use std::io;
use slog_json::Json;
use slog::{Drain, Logger};
use failure::Error;
use master_node::{Master, Settings};

fn run(logger: &Logger) -> Result<(), Error> {
    dotenv::dotenv().ok();

    let config = Settings::from_env()?;
    info!(logger, "Started the master node"; &config);

    let master = Master::new(&config, logger);
    master.update()?;

    Ok(())
}

fn main() {
    let drain = Json::default(io::stderr()).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());

    if env::args().count() > 1 {
        eprintln!("This is a stand-alone program and doesn't accept any arguments");
        eprintln!("with all configuration done via environment variables.");
        eprintln!();
        eprintln!("The defaults are: {:#?}", Settings::default());
        eprintln!();
        eprintln!("Aborting...");
        process::exit(1);
    }

    if let Err(e) = run(&logger) {
        error!(logger, "An error occurred"; "error-message" => e.to_string());

        for cause in e.causes().skip(1) {
            error!(logger, "Caused By"; "cause" => cause.to_string());
        }

        eprintln!("{}", e.backtrace());
        drop(logger);
        process::exit(1);
    }
}

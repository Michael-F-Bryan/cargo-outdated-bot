extern crate failure;
extern crate outdated_runner;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate slog_term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use slog::{Drain, Logger};
use failure::{Error, ResultExt};
use outdated_runner::Crate;

fn run(args: &Args, logger: Logger) -> Result<(), Error> {
    let crate_dir = match args.crate_dir.clone() {
        Some(d) => d,
        None => env::current_dir().context("Unable to fetch current directory")?,
    };

    let krate = Crate::from_dir(crate_dir);

    let results = outdated_runner::check_if_outdated(&krate, logger)?;

    Ok(())
}

fn main() {
    let args = Args::from_args();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root = slog::Logger::root(
        drain,
        o!(
            "version" => env!("CARGO_PKG_VERSION"), 
            "name" => env!("CARGO_PKG_NAME"),
           ),
    );

    info!(root, "Application Started"; "crate-dir" => format!("{:?}", args.crate_dir));

    if let Err(e) = run(&args, root.clone()) {
        let cause_chain = e.causes()
            .skip(1)
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        error!(root, "Application Failed"; 
            "backtrace" => e.backtrace().to_string(),
            "error" => e.to_string(),
            "causes" => cause_chain,
            );
        process::exit(1);
    }
}

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(short = "c", long = "crate-dir", help = "Path to the crate to check",
                parse(from_os_str))]
    crate_dir: Option<PathBuf>,
}

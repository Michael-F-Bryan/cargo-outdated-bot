extern crate failure;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate uuid;

use std::sync::Mutex;
use std::process;
use structopt::StructOpt;
use slog::{Drain, Logger};
use failure::Error;
use uuid::Uuid;

fn run(args: &Args, logger: Logger) -> Result<(), Error> {
    unimplemented!()
}

fn main() {
    let args = Args::from_args();
    let builder_id = Uuid::new_v4();

    let root = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!(
            "version" => env!("CARGO_PKG_VERSION"), 
            "name" => env!("CARGO_PKG_NAME"),
            "builder-id" => builder_id.to_string(),
           ),
    );

    info!(root, "Application Started"; "args" => format_args!("{:#?}", args));

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
struct Args {}

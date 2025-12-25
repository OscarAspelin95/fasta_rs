use clap::Parser;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

use fasta_rs::{args::App, common::AppError};

mod dispatch;
use dispatch::dispatch;

fn main() -> Result<(), AppError> {
    SimpleLogger::new().init().unwrap();

    let args = App::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.global_opts.threads)
        .build_global()
        .expect("Failed to configure global thread pool.");

    dispatch(args)?;

    Ok(())
}

use clap::Parser;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

mod amplicon;
mod common;
mod compress;
mod dispatch;
mod extract;
mod fa2tab;
mod filter;
mod head;
mod homopolymers;
mod reverse;
mod sample;
mod shuffle;
mod sort;
mod split;
mod stats;

use dispatch::dispatch;

mod args;
use args::App;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = App::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.global_opts.threads)
        .build_global()
        .expect("Failed to configure global thread pool.");

    dispatch(args);
}

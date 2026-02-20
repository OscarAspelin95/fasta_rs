use clap::Parser;
use log::error;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

mod amplicon;
mod args;
mod chunk;
mod compress;
mod dispatch;
mod errors;
mod extract;
mod fa2tab;
mod filter;
mod grep;
mod head;
mod homopolymers;
mod reverse;
mod sample;
mod shuffle;
mod sort;
mod split;
mod stats;

use args::App;
use dispatch::dispatch;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = App::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.global_opts.threads)
        .build_global()
        .expect("Failed to configure global thread pool.");

    let result = dispatch(args);

    match result {
        Ok(_) => {}
        Err(e) => error!("Error: {}", e),
    }
}

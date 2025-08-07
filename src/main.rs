use clap::Parser;
use simple_logger::SimpleLogger;

mod amplicon;
mod common;
mod extract;
mod fa2tab;
mod filter;
mod head;
mod homopolymers;
mod sort;
mod split;
mod stats;

mod dispatch;
use dispatch::dispatch;

mod args;
use args::App;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = App::parse();

    dispatch(args);
}

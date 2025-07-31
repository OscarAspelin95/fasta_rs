use clap::Parser;
use simple_logger::SimpleLogger;

mod common;
mod fa2tab;
mod head;
mod homopolymers;
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

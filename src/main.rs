use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct App {
    #[clap(subcommand)]
    command: SubCommand,

    #[clap(flatten)]
    global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[clap(short, long, global = true, required = false, default_value_t = 8)]
    threads: usize,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    Stats {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(long, default_value_t = 0)]
        min_len: usize,

        #[clap(long, default_value_t = usize::MAX)]
        max_len: usize,

        #[clap(short, long)]
        outfile: PathBuf,
    },
    Sort {},
    Sample {},
    Shuffle {},
}

#[allow(unused)]
fn main() {
    let args = App::parse();

    println!("{:?}", args);
}

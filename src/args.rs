use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct App {
    #[clap(subcommand)]
    pub command: SubCommand,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(short, long, global = true, required = false, default_value_t = 8)]
    pub threads: usize,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
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
    Fa2Tab {},
    Sort {},
    Sample {},
    Shuffle {},
    Head {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long)]
        num_seqs: usize,
    },
}

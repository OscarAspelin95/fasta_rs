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
    #[clap(
        short,
        long,
        global = true,
        required = false,
        default_value_t = 8,
        help = "Not applicable to all subcommands."
    )]
    pub threads: usize,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Stats {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value = "stats.json")]
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

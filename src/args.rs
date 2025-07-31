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
    Fa2tab {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value = "fa2tab.tsv")]
        outfile: PathBuf,
    },
    Homopolymers {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value_t = 5)]
        min_hp_len: usize,

        #[clap(short, long)]
        strict: bool,

        #[clap(short, long, default_value = "homopolymers.tsv")]
        outfile: PathBuf,
    },
    Query {},
    Sample {},
    Sort {},
    Shuffle {},
    Head {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long)]
        num_seqs: usize,
    },
    Amplicon {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long)]
        primers: PathBuf,

        #[clap(short, long, default_value = "amplicons.tsv")]
        outfile: PathBuf,
    },
}

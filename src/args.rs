use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum SearchType {
    Exact,
    Fuzzy,
}

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
    Split {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value = "fasta_split")]
        outdir: PathBuf,
    },
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
    Filter {
        #[clap(short, long)]
        fasta: PathBuf,

        // Length filter.
        #[clap(long, default_value_t = 0)]
        min_len: usize,

        #[clap(long, default_value_t = usize::MAX)]
        max_len: usize,

        // GC filter.
        #[clap(long, default_value_t = 0.0)]
        min_gc: f32,

        #[clap(long, default_value_t = 1.0)]
        max_gc: f32,

        // Ambig filter.
        #[clap(long, default_value_t = 0.0)]
        min_ambig: f32,

        #[clap(long, default_value_t = 1.0)]
        max_ambig: f32,

        // Softmask filter.
        #[clap(long, default_value_t = 0.0)]
        min_softmask: f32,

        #[clap(long, default_value_t = 1.0)]
        max_softmask: f32,

        // Shannon Entropy filter.
        #[clap(long, default_value_t = 0.0)]
        min_entropy: f32,

        #[clap(long, default_value_t = 100.0)]
        max_entropy: f32,

        #[clap(short, long, default_value = "filter.fasta")]
        outfile: PathBuf,
    },
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

        #[clap(short, long)]
        search_type: SearchType,

        #[clap(short, long, default_value = "amplicons.tsv")]
        outfile: PathBuf,
    },
}

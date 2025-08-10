use std::{path::PathBuf, usize};

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum SearchType {
    Exact,
    Fuzzy,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortType {
    Length,
    Id,
    Gc,
    Entropy,
    Softmask,
    Ambiguous,
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
    Extract {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value_t = 0)]
        start: usize,

        #[clap(short, long, default_value_t = usize::MAX)]
        end: usize,

        #[clap(short, long, default_value = "extract.fasta")]
        outfile: PathBuf,
    },
    Sample {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long, default_value_t = 1.0)]
        by: f32,

        #[clap(short, long, default_value = "sample.fasta")]
        outfile: PathBuf,
    },
    Sort {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(value_enum, short, long, default_value_t = SortType::Length)]
        by: SortType,

        #[clap(short, long)]
        reverse: bool,

        #[clap(short, long)]
        outfile: PathBuf,
    },
    Shuffle {
        #[clap(short, long)]
        fasta: PathBuf,

        #[clap(short, long)]
        outfile: PathBuf,
    },
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

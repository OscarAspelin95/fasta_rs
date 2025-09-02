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
        default_value_t = 0,
        help = "Not applicable to all subcommands. By default set to 0, meaning Rayon will choose automatically."
    )]
    pub threads: usize,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Split {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value = "fasta_split")]
        outdir: PathBuf,
    },
    Stats {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Fa2tab {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Homopolymers {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value_t = 5)]
        min_hp_len: usize,

        #[clap(short, long)]
        strict: bool,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Filter {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

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

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Extract {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value_t = 0)]
        start: usize,

        #[clap(short, long, default_value_t = usize::MAX)]
        end: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Sample {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value_t = 1.0)]
        by: f32,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Sort {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(value_enum, short, long, default_value_t = SortType::Length)]
        by: SortType,

        #[clap(short, long)]
        reverse: bool,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Shuffle {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Head {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value_t = 5)]
        num_seqs: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Grep {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        pattern: String,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Amplicon {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long)]
        primers: PathBuf,

        #[clap(short, long)]
        search_type: SearchType,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Compress {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long, default_value_t = 5)]
        max_hp_len: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Reverse {
        #[clap(short, long)]
        fasta: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
}

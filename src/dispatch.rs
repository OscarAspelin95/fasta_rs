use crate::args::{App, SubCommand};

use crate::amplicon::fasta_amplicon;
use crate::compress::fasta_compress;
use crate::extract::fasta_extract;
use crate::fa2tab::fasta_fa2tab;
use crate::filter::fasta_filter;
use crate::grep::fasta_grep;
use crate::head::fasta_head;
use crate::homopolymers::fasta_homopolymers;
use crate::reverse::fasta_reverse;
use crate::sample::fasta_sample;
use crate::shuffle::fasta_shuffle;
use crate::sort::fasta_sort;
use crate::split::fasta_split;
use crate::stats::fasta_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Split { fasta, outdir } => fasta_split(fasta, &outdir).unwrap(),
        SubCommand::Stats { fasta, outfile } => fasta_stats(fasta, outfile).unwrap(),
        SubCommand::Fa2tab { fasta, outfile } => fasta_fa2tab(fasta, outfile).unwrap(),
        SubCommand::Head {
            fasta,
            num_seqs,
            outfile,
        } => fasta_head(fasta, num_seqs, outfile).unwrap(),
        SubCommand::Grep {
            fastq,
            pattern,
            outfile,
        } => fasta_grep(fastq, pattern, outfile).unwrap(),
        SubCommand::Homopolymers {
            fasta,
            min_hp_len,
            strict,
            outfile,
        } => fasta_homopolymers(fasta, min_hp_len, strict, outfile).unwrap(),
        SubCommand::Sort {
            fasta,
            by,
            reverse,
            outfile,
        } => fasta_sort(fasta, by, reverse, outfile).unwrap(),
        SubCommand::Shuffle { fasta, outfile } => fasta_shuffle(fasta, outfile).unwrap(),
        SubCommand::Filter {
            fasta,
            min_len,
            max_len,
            min_gc,
            max_gc,
            min_ambig,
            max_ambig,
            min_softmask,
            max_softmask,
            min_entropy,
            max_entropy,
            outfile,
        } => fasta_filter(
            fasta,
            min_len,
            max_len,
            min_gc,
            max_gc,
            min_ambig,
            max_ambig,
            min_softmask,
            max_softmask,
            min_entropy,
            max_entropy,
            outfile,
        )
        .unwrap(),
        SubCommand::Extract {
            fasta,
            start,
            end,
            outfile,
        } => fasta_extract(fasta, start, end, outfile).unwrap(),
        SubCommand::Sample { fasta, by, outfile } => fasta_sample(fasta, by, outfile).unwrap(),
        SubCommand::Amplicon {
            fasta,
            primers,
            search_type,
            outfile,
        } => fasta_amplicon(fasta, &primers, &search_type, outfile).unwrap(),
        SubCommand::Compress {
            fasta,
            max_hp_len,
            outfile,
        } => fasta_compress(fasta, max_hp_len, outfile).unwrap(),
        SubCommand::Reverse { fasta, outfile } => fasta_reverse(fasta, outfile).unwrap(),
    };
}

use fasta_rs::amplicon::fasta_amplicon;
use fasta_rs::args::{App, SubCommand};
use fasta_rs::common::AppError;
use fasta_rs::compress::fasta_compress;
use fasta_rs::extract::fasta_extract;
use fasta_rs::fa2tab::fasta_fa2tab;
use fasta_rs::filter::fasta_filter;
use fasta_rs::grep::fasta_grep;
use fasta_rs::head::fasta_head;
use fasta_rs::homopolymers::fasta_homopolymers;
use fasta_rs::reverse::fasta_reverse;
use fasta_rs::sample::fasta_sample;
use fasta_rs::shuffle::fasta_shuffle;
use fasta_rs::sort::fasta_sort;
use fasta_rs::split::fasta_split;
use fasta_rs::stats::fasta_stats;

pub fn dispatch(args: App) -> Result<(), AppError> {
    match args.command {
        SubCommand::Split { fasta, outdir } => fasta_split(fasta, &outdir)?,
        SubCommand::Stats { fasta, outfile } => {
            let _ = fasta_stats(fasta, outfile)?;
        }
        SubCommand::Fa2tab { fasta, outfile } => fasta_fa2tab(fasta, outfile)?,
        SubCommand::Head {
            fasta,
            num_seqs,
            outfile,
        } => fasta_head(fasta, num_seqs, outfile)?,
        SubCommand::Grep {
            fastq,
            pattern,
            outfile,
        } => fasta_grep(fastq, pattern, outfile)?,
        SubCommand::Homopolymers {
            fasta,
            min_hp_len,
            strict,
            outfile,
        } => fasta_homopolymers(fasta, min_hp_len, strict, outfile)?,
        SubCommand::Sort {
            fasta,
            by,
            reverse,
            outfile,
        } => fasta_sort(fasta, by, reverse, outfile)?,
        SubCommand::Shuffle { fasta, outfile } => fasta_shuffle(fasta, outfile)?,
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
        )?,
        SubCommand::Extract {
            fasta,
            start,
            end,
            outfile,
        } => fasta_extract(fasta, start, end, outfile)?,
        SubCommand::Sample { fasta, by, outfile } => fasta_sample(fasta, by, outfile)?,
        SubCommand::Amplicon {
            fasta,
            primers,
            search_type,
            outfile,
        } => fasta_amplicon(fasta, &primers, &search_type, outfile)?,
        SubCommand::Compress {
            fasta,
            max_hp_len,
            outfile,
        } => fasta_compress(fasta, max_hp_len, outfile)?,
        SubCommand::Reverse { fasta, outfile } => fasta_reverse(fasta, outfile)?,
    };

    Ok(())
}

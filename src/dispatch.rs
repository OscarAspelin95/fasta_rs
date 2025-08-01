use crate::args::{App, SubCommand};

use crate::amplicon::fasta_amplicon;
use crate::fa2tab::fasta_fa2tab;
use crate::head::fasta_head;
use crate::homopolymers::fasta_homopolymers;
use crate::split::fasta_split;
use crate::stats::fasta_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Split { fasta, outdir } => fasta_split(&fasta, &outdir).unwrap(),
        SubCommand::Stats { fasta, outfile } => fasta_stats(&fasta, &outfile).unwrap(),
        SubCommand::Fa2tab { fasta, outfile } => fasta_fa2tab(&fasta, &outfile).unwrap(),
        SubCommand::Head { fasta, num_seqs } => fasta_head(&fasta, num_seqs).unwrap(),
        SubCommand::Homopolymers {
            fasta,
            min_hp_len,
            strict,
            outfile,
        } => fasta_homopolymers(&fasta, min_hp_len, strict, &outfile).unwrap(),
        SubCommand::Amplicon {
            fasta,
            primers,
            outfile,
        } => fasta_amplicon(&fasta, &primers, &outfile).unwrap(),
        _ => todo!(),
    };
}

use crate::args::{App, SubCommand};

use crate::head::fasta_head;
use crate::homopolymers::fasta_homopolymers;
use crate::stats::fasta_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Stats { fasta, outfile } => fasta_stats(&fasta, &outfile).unwrap(),
        SubCommand::Head { fasta, num_seqs } => fasta_head(&fasta, num_seqs).unwrap(),
        SubCommand::Homopolymers {
            fasta,
            min_hp_len,
            strict,
            outfile,
        } => fasta_homopolymers(&fasta, min_hp_len, strict, &outfile).unwrap(),
        _ => todo!(),
    };
}

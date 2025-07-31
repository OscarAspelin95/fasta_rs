use crate::args::{App, SubCommand};

use crate::head::fasta_head;
use crate::stats::fasta_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Stats {
            fasta,
            min_len,
            max_len,
            outfile,
        } => fasta_stats(&fasta, min_len, max_len, &outfile),

        SubCommand::Head { fasta, num_seqs } => fasta_head(&fasta, num_seqs),
        _ => todo!(),
    };
}

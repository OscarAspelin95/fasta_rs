use crate::common::{
    AppError, gc_content, get_bufwriter, needletail_fastx_reader, nucleotide_counts,
    shannon_entropy, utils::nucleotide_probabilities,
};
use std::{io::Write, path::PathBuf};

pub fn fasta_fa2tab(fasta: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;

    let mut writer = get_bufwriter(outfile)?;

    writer.write_all(b"id\tlength\tgc_content\tentropy\tnum_softmasked\tnum_ambiguous\n")?;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_seq = record.seq();
        let record_len = record_seq.len();
        let gc_content: f32 = gc_content(&record_seq);

        // Entropy
        let (canonical, num_softmasked, num_ambiguous) = nucleotide_counts(&record_seq);
        let probs = nucleotide_probabilities(&canonical);
        let entropy = shannon_entropy(&probs);

        // Id.
        writer.write_all(record.id())?;
        writer.write_all(b"\t")?;

        // Length.
        writer.write_all(record_len.to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Gc content.
        writer.write_all(gc_content.to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Entropy
        writer.write_all(entropy.to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Softmasked.
        writer.write_all(num_softmasked.to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Ambiguous.
        writer.write_all(num_ambiguous.to_string().as_bytes())?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}

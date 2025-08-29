pub mod reader;
pub use reader::{bio_fasta_reader, needletail_fastx_reader};

pub mod errors;
pub use errors::AppError;

pub mod writer;
pub use writer::{get_bufwriter, write_json};

pub mod utils;
pub use utils::{gc_content, nucleotide_counts, reverse_complement, shannon_entropy, usize_sub};

pub mod macros;

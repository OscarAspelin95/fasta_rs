pub mod reader;
pub use reader::needletail_fasta_reader;

pub mod errors;
pub use errors::AppError;

pub mod writer;
pub use writer::{get_bufwriter, write_json};

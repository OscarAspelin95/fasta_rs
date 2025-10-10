//! `fasta_rs` — general purpose toolkit for processing and manipulating fasta files.
//!
//! This crate provides utilities for common operations such as filtering, searching, sorting, etc.
//!
//! # Example
//! ```
//! use fasta_rs::stats::fasta_stats;
//!
//! let fasta = PathBuf::from("contigs.fasta");
//! let outfile = PathBuf::from("stats.json");
//! fasta_stats(Some(fasta), Some(outfile)).unwrap();
//! ```
//!
//! See the documentation for details about each command.
pub mod amplicon;
pub mod args;
pub mod common;
pub mod compress;
pub mod extract;
pub mod fa2tab;
pub mod filter;
pub mod grep;
pub mod head;
pub mod homopolymers;
pub mod reverse;
pub mod sample;
pub mod shuffle;
pub mod sort;
pub mod split;
pub mod stats;

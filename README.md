# fasta_rs
🚧 Work in progress fasta toolkit, aiming to be light version of [seqkit](https://github.com/shenwei356/seqkit/).

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the fasta_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/fasta_rs`.

## Usage
Run with:<br>
`fasta_rs <subcommand> <args>`

## Subcommands
### fasta_rs stats
🚧 Calculate basic stats such as num sequences, total length, gc content, etc.

`fasta_rs stats --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stats.json] - Output file.
</pre>

### fasta_rs fa2tab
🚧 Generate a .tsv file with basic information about each sequence.

`fasta_rs fa2tab --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stats.tsv] - Output file.
</pre>

### fasta_rs homopolymners
🚧 Find homopolymers in sequences.

`fasta_rs homopolymers --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--min-hp-len</b> [5] - Min homopolymer length to consider.
<b>--strict</b> [false] - Treat uppercase and lowercase nucleotides as different. E.g., AAAAA and aaaaa will be considered separate.
<b>-o/--outfile</b> [homopolymers.tsv] - Output file.
</pre>

### fasta_rs query
🚧 Query/filter sequences based on certain criteria.

`fasta_rs query --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--min-len</b> [0] - Minimum sequence length.

<b>--max-len</b> [u64::MAX] - Maximum sequence length.

<b>--min-gc</b> [0.0] - Minimum GC content.

<b>--max-gc</b> [1.0] - Maximum GC content.

<b>--min-ambig</b> [0.0] - Minimum fraction ambiguous bases.

<b>--max-ambig</b> [1.0] - Maximum fraction ambiguous bases.

<b>--min-softmask</b> [0.0] - Minimum fraction softmasked bases.

<b>--max-ambig</b> [1.0] - Maximum fraction softmaskes bases.

<b>-o/--outfile</b> [query.fasta] - Output file.
</pre>

### fasta_rs sample
🚧 (down)sample sequences based on a number of proportion.

`fasta_rs sample --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-n/--num_seqs</b> [1] - Num seqs to keep.

<b>--frac_seqs</b> [0.5] - Frac seqs to keep.

<b>-o/--outfile</b> [sample.fasta] - Output file.
</pre>

### fasta_rs sort
🚧 Sort sequences by a given metric.


`fasta_rs sort --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--by_len</b> [false] - Sort by ascending length.

<b>--by_id</b> [false] - Sort by ascending sequence id.

<b>--by_gc</b> [false] - Sort by ascending gc content.

<b>-r/--reverse</b> [false] - Sort in descending order.

<b>-o/--outfile</b> [sorted.fasta] - Output file.
</pre>

### fasta_rs shuffle
🚧 Randomly shuffle sequences.


`fasta_rs shuffle --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--by_len</b> [false] - Sort by ascending length.

<b>--by_id</b> - Sort by ascending sequence id.

<b>--by_gc</b> [false] - Sort by ascending gc content.

<b>-r/--reverse</b> - Sort in descending order.

<b>-o/--outfile</b> [shuffled.fasta] - Output file.
</pre>

### fasta_rs head
🚧 View the first n sequences.

`fasta_rs head --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-n/--num_seqs</b> [5] - Number of sequences to output.
</pre>

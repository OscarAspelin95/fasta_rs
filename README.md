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
<b>--min_len</b> [0] - Min len to consider sequence.

<b>--max_len</b> [u64::MAX] - Max len to consider sequence.

<b>--outfile</b> [stats.json] - Output file.

</pre>

### fasta_rs sample
🚧 (down)sample sequences based on a number of proportion.

`fasta_rs sample --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--num_seqs</b> [1] - Num seqs to keep.

<b>--frac_seqs</b> [0.5] - Frac seqs to keep.

<b>--outfile</b> [sample.fasta] - Output file.

</pre>

### fasta_rs sort
🚧 Sort sequences by a given metric.


`fasta_rs sort --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--by_len</b> [false] - Sort by ascending length.

<b>--by_gc</b> [false] - Sort by ascending gc content.

<b>--by_id</b> [false] - Sort by ascending sequence id.

<b>--reverse</b> [false] - Sort in descending order.

<b>--outfile</b> [sorted.fasta] - Output file.
</pre>

### fasta_rs shuffle
🚧 Randomly shuffle sequences.


`fasta_rs shuffle --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--by_len</b> [false] - Sort by ascending length.

<b>--by_gc</b> [false] - Sort by ascending gc content.

<b>--by_id</b> - Sort by ascending sequence id.

<b>--reverse</b> - Sort in descending order.

<b>--outfile</b> [shuffled.fasta] - Output file.
</pre>

### fasta_rs head
🚧 View the first n sequences.

`fasta_rs head --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--num_seqs</b> [5] - Number of sequences to output.
</pre>

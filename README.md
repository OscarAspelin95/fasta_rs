# fasta_rs
🚧 Work in progress fasta toolkit, aiming to an alternative to [seqkit](https://github.com/shenwei356/seqkit/).

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the fasta_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/fasta_rs`.

## Usage
Run with:<br>
`fasta_rs <subcommand> <args>`<br>

## Example
The following command will randomly sample 50% of the sequences, filter by gc content and finally convert to a .tsv file.<br>
`fasta_rs sample -b 0.5 < file.fasta | fasta_rs filter --min-gc 0.5 | fasta_rs fa2tab > out.tsv`

## ToDo
- [ ] Improve readability for writer.
- [ ] Add buffer flush to writers.
- [ ] Rayon global threadpool for provided --threads arg.

## Subcommands
🔴 Not implemented yet (but planning to).<br>
🟡 Implemented but not tested/fully featured.<br>
🟢 Beta-mode available!

### fasta_rs `split`
🟢 Split into one file per sequence.

`fasta_rs split --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outdir</b> [fasta_split] - Output directory.
</pre>

### fasta_rs `stats`
🟢 Calculate basic stats.

`fasta_rs stats --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `fa2tab`
🟢 Generate a .tsv file with basic information about each sequence.

`fasta_rs fa2tab --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `homopolymers`
🟢 Find homopolymers in sequences.

`fasta_rs homopolymers --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-m/--min-hp-len</b> [5] - Min homopolymer length to consider.

<b>-s/--strict</b> [false] - Only consider homopolymers for {A, C, G, T, a, c, g, t}.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `filter`
🟢 Filter sequences based on certain criteria.

`fasta_rs filter --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--min-len</b> [0] - Minimum sequence length.

<b>--max-len</b> [u64::MAX] - Maximum sequence length.

<b>--min-gc</b> [0.0] - Minimum GC content.

<b>--max-gc</b> [1.0] - Maximum GC content.

<b>--min-ambig</b> [0.0] - Minimum fraction ambiguous bases.

<b>--max-ambig</b> [1.0] - Maximum fraction ambiguous bases.

<b>--min-softmask</b> [0.0] - Minimum fraction softmasked bases.

<b>--max-softmask</b> [1.0] - Maximum fraction softmaskes bases.

<b>--min-entropy</b> [0.0] - Minimum Shannon Entropy.

<b>--max-entropy</b> [100.0] - Maximum Shannon Entropy.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `extract`
🟢 Extract sub-sequence based on provided range.

`fasta_rs extract --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-s/--start</b> [0] - Start coordinate (BED offset).

<b>-e/--end</b> [u64::MAX] - End coordinate (BED offset).

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

Since the coordinates are BED-compatible, extracting the ith base would be equivalent to using `-s i-1` and `-e i`

### fasta_rs `sample`
🟢 (down)sample sequences based on a number or proportion.

`fasta_rs sample --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-b/--by</b> [1.0] - Num/fraction seqs to keep.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `sort`
🟢 Sort sequences by a given metric.

`fasta_rs sort --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>--by</b> [length] - {length, id, gc, entropy, softmask, ambiguous}.

<b>-r/--reverse</b> [false] - Sort in descending order.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `shuffle`
🟢 Randomly shuffle sequences.

`fasta_rs shuffle --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `head`
🟢 View the first n sequences.

`fasta_rs head --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-n/--num_seqs</b> [5] - Number of sequences to output.
</pre>

### fasta_rs `amplicon`
🟢 In silico PCR by exact or fuzzy primer matching.

`fasta_rs amplicon --fasta <sequences.fasta> --primers <primers.tsv> --search-type {exact, fuzzy} <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

#### primer file
The primer.tsv TAB separated file needs to specifies the following for each primer pair:
- Primer name.
- Forward primer sequence (5' -> 3').
- Reverse primer sequence (5' -> 3').
- Expected minimum length of insert size.
- Expected maximum length of insert size.
- Num allowed mismatches (only for fuzzy search).

### fasta_rs `compress`
🟢 Homopolymer compress sequences.

`fasta_rs compress --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-m/--max-hp-len</b> [5] - Compress down to homopolymers of max provided length. E.g., ATCGGGGGGG with -m 3 outputs ATCGGG.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fasta_rs `reverse`
🟢 Reverse complement sequences.

`fasta_rs reverse --fasta <sequences.fasta> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

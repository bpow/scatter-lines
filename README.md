# Scatter/distribute (chunks of) lines in a file into other files

## Why?

You've got a big file, and want several smaller files, maybe of similar size.
You could use `split` if you knew exactly how many lines you want per file,
or if the file is so big (or coming from a pipe) and you don't want to read
the whole thing to find out where to split the lines.

My specific motivating use case was splitting up fastq files-- sets of unaligned
genetic sequence read records that come in line groups of 4 where the order
doesn't particularly matter. Using this, I can read in chunks that are multiples
of 4 lines (e.g., 4, 8, 16, ...) and be sure to get complete FASTQ records but
in the end, the file sizes will have close-to-the-same number of lines.

## Building

Pretty standard Rust... `cargo build --release` will put an executable in the
`target/release` directory. If you've set up a static target in cargo, then something
like `cargo build --target=x86_64-unknown-linux-musl --release` will give you a
static executable. Or use a release version...

## Usage

```
Usage: scatter-lines [OPTIONS] [OUTPUT]...

Arguments:
  [OUTPUT]...  Paths to output files

Options:
      --input <INPUT>            Path to the input file (default: stdin)
      --chunk-size <CHUNK_SIZE>  Number of contiguous lines per chunk (default: 256) [default: 256]
  -h, --help                     Print help
  -V, --version                  Print version
```

Want to read from a compressed file and write to compressed outputs? Try something
like this:

```bash
zcat input.fastq.gz | scatter-lines --chunk-size 4 >(gzip -c > split1.fastq.gz) >(gzip -c split2.fastq.gz) >(gzip-c > split3.fastq.gz)
```

That way, the gzip part of things takes place in a different process. Instant multithreading!

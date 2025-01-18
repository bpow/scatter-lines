use clap::{Parser};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};

#[derive(Parser, Debug)]
#[command(name = "Line Distributor")]
#[command(version = "1.0")]
#[command(about = "Distributes chunks of lines from an input file or stdin to multiple output files")]
struct Args {
    /// Paths to output files
    output: Vec<String>,

    /// Path to the input file (default: stdin)
    #[arg(long)]
    input: Option<String>,

    /// Number of contiguous lines per chunk (default: 256)
    #[arg(long, default_value_t = 256)]
    chunk_size: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let num_outputs = args.output.len();

    // Determine if input is from stdin or a file
    let reader: Box<dyn BufRead> = match &args.input {
        Some(path) => {
            let input_file = File::open(path)?;
            Box::new(io::BufReader::new(input_file))
        }
        None => Box::new(io::BufReader::new(io::stdin())),
    };

    let mut output_writers: Vec<BufWriter<File>> = args.output.into_iter().map(|output_file| {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file).unwrap();
        let writer = BufWriter::new(file);
        writer
    }).collect();

    let mut buffer: Vec<String> = Vec::new();
    let mut output_index = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the result of reading the line
        buffer.push(line);

        // If the buffer has reached the chunk size, write the chunk to the current output file
        if buffer.len() == args.chunk_size {
            let output_writer = &mut output_writers[output_index];
            for buffered_line in buffer.drain(..) {
                writeln!(output_writer, "{}", buffered_line).expect("Failed to write to output file");
            }
            // Move to the next output file in round-robin fashion
            output_index = (output_index + 1) % num_outputs;
        }
    }

    // If there are any remaining lines in the buffer, write them to the current output file
    if !buffer.is_empty() {
        let output_writer = &mut output_writers[output_index];
        for buffered_line in buffer {
            writeln!(output_writer, "{}", buffered_line)?;
        }
    }

    Ok(())
}


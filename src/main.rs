use clap::{Parser};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(name = "Line Distributor")]
#[command(version = "1.0")]
#[command(about = "Distributes chunks of lines from an input file or stdin to multiple output files")]
struct Args {
    /// Path to the input file (default: stdin)
    #[arg(long)]
    input: Option<String>,

    /// Number of output files to generate (default: 4)
    #[arg(long, default_value_t = 4)]
    num_output_files: usize,

    /// Template for output file names (default: output{}.txt)
    #[arg(long, default_value = "output{}.txt")]
    output_template: String,

    /// Number of contiguous lines per chunk (default: 1)
    #[arg(long, default_value_t = 1)]
    chunk_size: usize,
}

fn main() -> io::Result<()> {
    // Parse the command-line arguments using clap's derive feature
    let args = Args::parse();

    // Determine if input is from stdin or a file
    let reader: Box<dyn BufRead> = match &args.input {
        Some(path) => {
            let input_file = File::open(path)?;
            Box::new(io::BufReader::new(input_file))
        }
        None => Box::new(io::BufReader::new(io::stdin())),
    };

    // Generate output file names based on the template
    let mut output_files: Vec<GzEncoder<File>> = Vec::new();
    for i in 1..=args.num_output_files {
        let output_filename = args.output_template.replace("{}", &i.to_string());
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_filename)?;
        let encoder = GzEncoder::new(file, Compression::default());
        output_files.push(encoder);
    }

    // Process the input (from stdin or file) line by line
    let mut buffer: Vec<String> = Vec::new();
    let mut output_index = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the result of reading the line
        buffer.push(line);

        // If the buffer has reached the chunk size, write the chunk to the current output file
        if buffer.len() == args.chunk_size {
            let output_file = &mut output_files[output_index];
            for buffered_line in buffer.drain(..) {
                writeln!(output_file, "{}", buffered_line)?;
            }
            // Move to the next output file in round-robin fashion
            output_index = (output_index + 1) % args.num_output_files;
        }
    }

    // If there are any remaining lines in the buffer, write them to the current output file
    if !buffer.is_empty() {
        let output_file = &mut output_files[output_index];
        for buffered_line in buffer {
            writeln!(output_file, "{}", buffered_line)?;
        }
    }

    Ok(())
}


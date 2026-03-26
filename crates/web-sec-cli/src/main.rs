use std::{io, path};

use clap::{Parser};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'f', long = "file", value_name = "har-file")]
    har_file: Option<path::PathBuf>,

    /// Positional arguments: HAR FILE INPUT or INPUT when using -f
    #[arg(value_name = "ARGS", num_args = 0..=2)]
    args: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(har_file) = args.har_file {
        println!("Processing HAR file: {}", har_file.display());
    };

    Ok(())
}

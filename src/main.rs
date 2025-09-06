use std::{error::Error, fs::OpenOptions, io::{BufReader, Read}, path::PathBuf};

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "analysis of PS3/PS4 .sfo files", long_about = None)]
struct Args {
  #[arg(
    long,
    required = true,
    help = "Path to a .sfo file"
  )]
  input_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&args.input_file)?;

    let mut reader = BufReader::new(input_file);

    let mut magic: [u8; 4] = [0; 4];
    reader.read_exact(&mut magic)?;

    println!("Magic: {:02X?}", &magic);

    Ok(())
}

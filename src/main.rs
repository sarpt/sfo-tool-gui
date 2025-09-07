use std::{
  error::Error,
  fs::OpenOptions,
  io::{BufReader, Read},
  path::PathBuf,
};

use clap::Parser;
use thiserror::Error;

use crate::sfo::header::Header;

mod sfo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "analysis of PS3/PS4 .sfo files", long_about = None)]
struct Args {
  #[arg(long, required = true, help = "Path to a .sfo file")]
  input_file: PathBuf,
}

const UNCONTAINED_PARAM_SFO_MAGIC: [u8; 4] = [0x00, 0x50, 0x53, 0x46];

#[derive(Error, Debug)]
enum MainErr {
  #[error("Provided file doesn't match known .sfo files magic")]
  UnknownMagic,
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

  println!("Magic: {:#04X?}", &magic);

  if magic != UNCONTAINED_PARAM_SFO_MAGIC {
    println!(
      "Magic {:#04X?} doesn't match any known .sfo file magic",
      &magic
    );
    return Err(Box::new(MainErr::UnknownMagic));
  }

  let header = Header::new(&mut reader)?;
  println!("{header}");

  Ok(())
}

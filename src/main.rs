use std::{error::Error, fs::OpenOptions, io::BufReader, path::PathBuf};

use clap::Parser;

use crate::{gui::GuiApp, sfo::Sfo};

mod gui;
mod sfo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "analysis of PS3 .sfo files", long_about = None)]
struct Args {
  #[arg(long, required = true, help = "Path to a .sfo file")]
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
  let sfo = Sfo::new(&mut reader).map_err(Box::new)?;

  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "Read .sfo",
    native_options,
    Box::new(|cc| Ok(Box::new(GuiApp::new(cc, sfo)))),
  )
  .map_err(|err| format!("could not start eframe application: {err}"))?;

  Ok(())
}

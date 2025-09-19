use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::gui::GuiApp;

mod gui;
mod sfo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "analysis of PS3 .sfo files", long_about = None)]
struct Args {
  #[arg(long, required = false, help = "Path to a .sfo file")]
  input_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "Read .sfo",
    native_options,
    Box::new(|cc| Ok(Box::new(GuiApp::new(cc, args.input_file.as_ref())))),
  )
  .map_err(|err| format!("could not start eframe application: {err}"))?;

  Ok(())
}

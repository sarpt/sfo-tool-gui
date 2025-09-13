use eframe::egui;
use std::{
  error::Error,
  fs::OpenOptions,
  io::{BufReader, Read},
  path::PathBuf,
};

use clap::Parser;
use thiserror::Error;

use crate::sfo::{header::Header, index_table::IndexTable, mapping::Mapping};

mod sfo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "analysis of PS3 .sfo files", long_about = None)]
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

  let index_table = IndexTable::new(&mut reader, &header)?;
  println!("{index_table}");

  let entries_mapping = Mapping::new(&mut reader, &index_table)?;
  println!("{entries_mapping}");

  let native_options = eframe::NativeOptions::default();
  eframe::run_native(
    "Read .sfo",
    native_options,
    Box::new(|cc| Ok(Box::new(GuiApp::new(cc, entries_mapping)))),
  )
  .map_err(|err| format!("could not start eframe application: {err}"))?;

  Ok(())
}

struct GuiApp {
  mapping: Mapping,
}

impl GuiApp {
  fn new(_cc: &eframe::CreationContext<'_>, mapping: Mapping) -> Self {
    GuiApp { mapping }
  }

  fn mapping_entries_grid(&self, ui: &mut eframe::egui::Ui) {
    for (key, value) in &self.mapping.entries {
      ui.label(key.to_string());
      ui.label(value.to_string());
      ui.end_row();
    }
  }
}

impl eframe::App for GuiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::Grid::new("mapping_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
          self.mapping_entries_grid(ui);
        });
    });
  }
}

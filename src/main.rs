use eframe::egui;
use std::{error::Error, fs::OpenOptions, io::BufReader, path::PathBuf};

use clap::Parser;

use crate::sfo::Sfo;

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

struct GuiApp {
  sfo: Sfo,
}

impl GuiApp {
  fn new(_cc: &eframe::CreationContext<'_>, sfo: Sfo) -> Self {
    GuiApp { sfo }
  }

  fn mapping_entries_grid(&self, ui: &mut eframe::egui::Ui) {
    for (key, value) in &self.sfo.entries_mapping.entries {
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

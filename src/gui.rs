use std::{
  fs::OpenOptions,
  io::BufReader,
  path::{Path, PathBuf},
};

use crate::sfo::Sfo;
use eframe::egui::{self};
use rfd::FileDialog;

struct LoadedSfo {
  sfo: Sfo,
  path: PathBuf,
}

pub struct GuiApp {
  sfo: Option<LoadedSfo>,
}

impl GuiApp {
  pub fn new<T>(_cc: &eframe::CreationContext<'_>, path: Option<&T>) -> Self
  where
    T: AsRef<Path> + Into<PathBuf>,
  {
    let sfo = path.and_then(|path| {
      load_sfo_file(path.as_ref()).map_or_else(
        |err| {
          println!("could not load sfo file provided in arguments: {err}");
          None
        },
        |sfo| {
          Some(LoadedSfo {
            sfo,
            path: PathBuf::from(path.as_ref()),
          })
        },
      )
    });

    GuiApp { sfo }
  }

  fn show_loaded_file(&self, ctx: &egui::Context, loaded_sfo: &LoadedSfo) {
    egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
      ui.label(format!(
        "Loaded file: {}",
        loaded_sfo.path.to_string_lossy()
      ))
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::both().show(ui, |ui| {
        self.mapping_entries_grid(ui, &loaded_sfo.sfo);
      });
    });
  }

  fn show_no_file_loaded_info(&mut self, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center).with_main_justify(true),
        |ui| {
          let upload_button =
            ui.link("No .sfo file has been provided.\nClick here to provide a .sfo file");

          if upload_button.clicked() {
            self.show_load_sfo_picker(ctx);
          }
        },
      );
    });
  }

  fn show_file_loading_error(&mut self, ui: &mut eframe::egui::Ui) {}

  fn mapping_entries_grid(&self, ui: &mut eframe::egui::Ui, sfo: &Sfo) {
    egui::Grid::new("mapping_grid")
      .num_columns(2)
      .min_col_width(120.0)
      .max_col_width(ui.available_size().x)
      .spacing([40.0, 4.0])
      .striped(true)
      .show(ui, |ui| {
        ui.label("KEY");
        ui.label("DATA");
        ui.end_row();

        ui.label("");
        ui.add_sized(ui.available_size(), egui::Label::new(""));
        ui.end_row();

        for (key, entry) in sfo.iter() {
          ui.label(key.to_string())
            .on_hover_text(entry.index_table_entry.to_string());
          ui.label(entry.data.to_string());
          ui.end_row();
        }
      });
  }

  fn show_load_sfo_picker(&mut self, ctx: &egui::Context) {
    let files = FileDialog::new()
      .add_filter("Sfo", &["sfo", "SFO"])
      .set_directory("/")
      .pick_file();

    let data_path = match files {
      Some(path) => path,
      None => {
        println!("no path provided");
        return;
      }
    };
    self.sfo = load_sfo_file(&data_path).map_or_else(
      |err| {
        println!("could not load a sfo file: {err}");
        None
      },
      |sfo| {
        Some(LoadedSfo {
          sfo,
          path: PathBuf::from(&data_path),
        })
      },
    );
    ctx.request_repaint();
  }
}

impl eframe::App for GuiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    match &self.sfo {
      Some(sfo) => {
        self.show_loaded_file(ctx, sfo);
      }
      None => {
        self.show_no_file_loaded_info(ctx);
      }
    }
  }
}

fn load_sfo_file<T>(path: T) -> Result<Sfo, String>
where
  T: AsRef<Path>,
{
  let file = OpenOptions::new()
    .read(true)
    .write(false)
    .open(path.as_ref())
    .map_err(|err| format!("could not load file: {err}"))?;

  let mut reader = BufReader::new(file);
  Sfo::new(&mut reader).map_err(|err| format!("could not load file: {err}"))
}

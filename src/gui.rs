use std::{
  borrow::Cow,
  fs::OpenOptions,
  io::BufReader,
  path::{Path, PathBuf},
};

use crate::sfo::Sfo;
use eframe::egui::{self, Id};
use rfd::FileDialog;

struct LoadedSfo {
  sfo: Sfo,
  path: PathBuf,
}

pub struct GuiApp {
  err_msg: Option<String>,
  sfo: Option<LoadedSfo>,
}

const NO_SFO_FILE_MSG: &str = "No .sfo file has been provided";

impl GuiApp {
  pub fn new<T>(_cc: &eframe::CreationContext<'_>, path: Option<&T>) -> Self
  where
    T: AsRef<Path> + Into<PathBuf>,
  {
    let mut err_msg: Option<String> = None;
    let sfo = path.and_then(|path| {
      load_sfo_file(path.as_ref()).map_or_else(
        |err| {
          err_msg = Some(format!(
            "could not load sfo file with path {} provided in \"input-file\" argument: {err}",
            path.as_ref().to_string_lossy()
          ));
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

    GuiApp { sfo, err_msg }
  }

  fn show_header(&mut self, ctx: &egui::Context) {
    egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
      let upload_button = ui.button("Load .sfo file");
      if upload_button.clicked() {
        self.show_load_sfo_picker(ctx);
      }

      ui.label(format!(
        "Loaded file: {}",
        self
          .sfo
          .as_ref()
          .map_or(Cow::from(NO_SFO_FILE_MSG), |sfo| sfo.path.to_string_lossy())
      ));
    });
  }

  fn show_loaded_file(&self, ctx: &egui::Context, loaded_sfo: &LoadedSfo) {
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
          let upload_link = ui.link(format!(
            "{NO_SFO_FILE_MSG}\nClick here to provide a .sfo file"
          ));
          if upload_link.clicked() {
            self.show_load_sfo_picker(ctx);
          }
        },
      );
    });
  }

  fn handle_loading_error_modal(&mut self, ctx: &eframe::egui::Context) {
    if let Some(err_msg) = &self.err_msg {
      let modal = egui::Modal::new(Id::new("file_loading_error_modal")).show(ctx, |ui| {
        ui.set_width(250.0);
        ui.label(err_msg);
      });

      if modal.should_close() {
        self.err_msg = None;
      }
    }
  }

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
    let new_sfo = load_sfo_file(&data_path).map_or_else(
      |err| {
        self.err_msg = Some(format!("could not load a sfo file: {err}"));
        None
      },
      |sfo| {
        Some(LoadedSfo {
          sfo,
          path: PathBuf::from(&data_path),
        })
      },
    );

    if new_sfo.is_some() {
      self.sfo = new_sfo;
    }

    ctx.request_repaint();
  }
}

impl eframe::App for GuiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if self.err_msg.is_some() {
      self.handle_loading_error_modal(ctx);
    }

    if self.sfo.is_some() {
      self.show_header(ctx);
    }

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

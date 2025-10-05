use std::{
  borrow::Cow,
  fs::OpenOptions,
  io::{BufReader, BufWriter},
  path::{Path, PathBuf},
};

use crate::{gui::entry_update_modal::EntryUpdateModal, sfo::Sfo};
use eframe::egui::{self, Id};
use rfd::FileDialog;

pub mod entry_update_modal;

struct LoadedSfo {
  sfo: Sfo,
  path: PathBuf,
}

pub struct GuiApp {
  err_msg: Option<String>,
  sfo: Option<LoadedSfo>,
  entry_update_modal: Option<EntryUpdateModal>,
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

    GuiApp {
      sfo,
      err_msg,
      entry_update_modal: None,
    }
  }

  fn show_header(&mut self, ctx: &egui::Context) {
    egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
      ui.horizontal(|ui| {
        let load_sfo_btn = ui.button("Load .sfo file");
        if load_sfo_btn.clicked() {
          self.show_load_sfo_picker(ctx);
        }

        let save_sfo_btn = ui.add_enabled(self.sfo.is_some(), egui::Button::new("Save .sfo file")); // TODO: add 'modified' flag to LoadedSfo
        if save_sfo_btn.clicked() {
          self.show_save_sfo_picker();
        }
      });

      ui.label(format!(
        "Loaded file: {}",
        self
          .sfo
          .as_ref()
          .map_or(Cow::from(NO_SFO_FILE_MSG), |sfo| sfo.path.to_string_lossy())
      ));
    });
  }

  fn show_loaded_file(&mut self, ctx: &egui::Context, loaded_sfo: &LoadedSfo) {
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

  fn handle_err_msg_modal(&mut self, ctx: &eframe::egui::Context) {
    if let Some(err_msg) = &self.err_msg {
      let modal = egui::Modal::new(Id::new("err_msg_modal")).show(ctx, |ui| {
        ui.set_width(250.0);
        ui.label(err_msg);
      });

      if modal.should_close() {
        self.err_msg = None;
      }
    }
  }

  fn mapping_entries_grid(&mut self, ui: &mut eframe::egui::Ui, sfo: &Sfo) {
    egui::Grid::new("mapping_grid")
      .num_columns(3)
      .min_col_width(10.0)
      .max_col_width(ui.available_size().x)
      .spacing([20.0, 4.0])
      .striped(true)
      .show(ui, |ui| {
        let add_btn = ui.button("Add");
        if add_btn.clicked() {
          self.entry_update_modal = Some(EntryUpdateModal::default());
        }

        ui.label("KEY");
        ui.label("DATA");
        ui.end_row();

        ui.label("");
        ui.label("");
        ui.add_sized(ui.available_size(), egui::Label::new(""));
        ui.end_row();

        for (key, entry) in sfo.iter() {
          ui.label("");
          ui.label(key.to_string())
            .on_hover_text(entry.index_table_entry.to_string());
          ui.label(entry.data.to_string());
          ui.end_row();
        }
      });
  }

  fn show_save_sfo_picker(&mut self) {
    if let Some(sfo) = &self.sfo {
      let files = FileDialog::new()
        .add_filter("Sfo", &["sfo", "SFO"])
        .set_directory("/")
        .save_file();

      let path = match files {
        Some(path) => path,
        None => {
          println!("no path provided");
          return;
        }
      };

      let result = save_sfo_file(path, &sfo.sfo);
      if let Err(err_msg) = result {
        self.err_msg = Some(err_msg);
      }
    }
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
      self.handle_err_msg_modal(ctx);
    }

    if let Some(mut entry_update_modal) = self.entry_update_modal.take() {
      match entry_update_modal.show(ctx) {
        Ok(draft_entry) => match draft_entry {
          entry_update_modal::EntryUpdateModalAction::Close => {}
          entry_update_modal::EntryUpdateModalAction::Noop => {
            self.entry_update_modal = Some(entry_update_modal);
          }
          entry_update_modal::EntryUpdateModalAction::Save(entry) => {
            if let Some(sfo) = &mut self.sfo {
              sfo.sfo.add(entry.key, entry.field);
            }
          }
        },
        Err(err_msg) => {
          self.err_msg = Some(err_msg);
        }
      };
    }

    if self.sfo.is_some() {
      self.show_header(ctx);
    }

    let sfo = self.sfo.take();
    match sfo {
      Some(sfo) => {
        self.show_loaded_file(ctx, &sfo);
        self.sfo = Some(sfo);
      }
      None => {
        self.show_no_file_loaded_info(ctx);
      }
    }
  }
}

fn save_sfo_file<T>(path: T, sfo: &Sfo) -> Result<(), String>
where
  T: AsRef<Path>,
{
  let file = OpenOptions::new()
    .read(false)
    .write(true)
    .create(true)
    .truncate(true)
    .open(path.as_ref())
    .map_err(|err| format!("could not load file: {err}"))?;

  let mut writer = BufWriter::new(file);
  sfo
    .export(&mut writer)
    .map_err(|err| format!("could not save file: {}", err))
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

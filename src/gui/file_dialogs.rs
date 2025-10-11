use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};

use rfd::FileDialog;

use crate::{gui::file_ops::load_sfo_file, sfo::Sfo};

const FORMAT_NAME: &str = "System File Object";
const EXTENSIONS: [&str; 2] = ["sfo", "SFO"];

pub fn save_sfo_dialog(sfo: &Sfo) -> Result<(), String> {
  let file_dialog_result = FileDialog::new()
    .add_filter(FORMAT_NAME, &EXTENSIONS)
    .set_directory("/")
    .save_file();

  let path = match file_dialog_result {
    Some(mut path) => {
      if path.extension().is_none() {
        path.set_extension(EXTENSIONS[0]);
      }

      path
    }
    None => {
      return Err(String::from("No file has been selected"));
    }
  };

  let file = OpenOptions::new()
    .read(false)
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .map_err(|err| format!("could not load file: {err}"))?;

  let mut writer = BufWriter::new(file);
  sfo
    .export(&mut writer)
    .map_err(|err| format!("could not save file: {}", err))
}

pub fn load_sfo_dialog() -> Result<(Sfo, PathBuf), String> {
  let files = FileDialog::new()
    .add_filter(FORMAT_NAME, &EXTENSIONS)
    .set_directory("/")
    .pick_file();

  let path = match files {
    Some(path) => path,
    None => {
      return Err(String::from("No file has been selected"));
    }
  };

  let sfo = load_sfo_file(&path)?;
  Ok((sfo, path))
}

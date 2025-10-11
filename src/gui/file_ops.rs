use std::{fs::OpenOptions, io::BufReader, path::Path};

use crate::sfo::Sfo;

pub fn load_sfo_file<T>(path: T) -> Result<Sfo, String>
where
  T: AsRef<Path>,
{
  let file = OpenOptions::new()
    .read(true)
    .write(false)
    .open(&path)
    .map_err(|err| format!("could not load file: {err}"))?;

  let mut reader = BufReader::new(file);
  Sfo::new(&mut reader).map_err(|err| format!("could not load file: {err}"))
}

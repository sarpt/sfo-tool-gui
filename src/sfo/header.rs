use std::{
  fmt::Display,
  fs::File,
  io::{BufReader, Read},
};

pub struct Header {
  version: u32,
  key_table_start: u32,
  data_table_start: u32,
  table_entries: u32,
}

impl Header {
  pub fn new(reader: &mut BufReader<File>) -> Result<Self, String> {
    let mut buffer: [u8; 4] = [0; 4];
    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read version: {err}"))?;
    let version = u32::from_le_bytes(buffer);

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read key table start: {err}"))?;
    let key_table_start = u32::from_le_bytes(buffer);

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read data table start: {err}"))?;
    let data_table_start = u32::from_le_bytes(buffer);

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read table entries: {err}"))?;
    let table_entries = u32::from_le_bytes(buffer);

    Ok(Header {
      version,
      key_table_start,
      data_table_start,
      table_entries,
    })
  }
}

impl Display for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Version: {:#02X}\nKey table start offset: {:#02X}\nData table start offset: {:#02X}\nTable entries count: {} [{:#02X}]",
      self.version,
      self.key_table_start,
      self.data_table_start,
      self.table_entries,
      self.table_entries
    )
  }
}

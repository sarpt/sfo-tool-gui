use std::{fmt::Display, io::Read};

#[derive(Clone, Copy, Debug)]
pub struct Header {
  pub version: u32,
  pub key_table_start: u32,
  pub data_table_start: u32,
  pub table_entries: u32,
}

const KEY_TABLE_ENTRY_SIZE: u8 = 16;

impl Header {
  pub fn new<T>(reader: &mut T) -> Result<Self, String>
  where
    T: Read,
  {
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

  pub fn add_entry(&mut self, key_size: u32) {
    self.key_table_start += KEY_TABLE_ENTRY_SIZE as u32;
    self.data_table_start += key_size + KEY_TABLE_ENTRY_SIZE as u32;
    self.table_entries += 1;
  }
}

impl Display for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Version: {:#010X}\nKey table start offset: {:#010X}\nData table start offset: {:#010X}\nTable entries count: {} [{:#010X}]",
      self.version,
      self.key_table_start,
      self.data_table_start,
      self.table_entries,
      self.table_entries
    )
  }
}

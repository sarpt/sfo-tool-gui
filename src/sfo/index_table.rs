use std::{fmt::Display, io::Read};

use crate::sfo::{format::Format, header::Header};

pub struct IndexTable {
  pub entries: Vec<IndexTableEntry>,
}

impl IndexTable {
  pub fn new<T>(reader: &mut T, header: &Header) -> Result<Self, String>
  where
    T: Read,
  {
    let mut entries: Vec<IndexTableEntry> = Vec::new();

    for idx in 0..header.table_entries {
      let mut entry = IndexTableEntry::new(reader)?;

      if idx != 0 {
        let prev_entry = entries
          .get_mut((idx - 1) as usize)
          .ok_or_else(|| format!("could not update length of key {}", idx - 1))?;
        prev_entry.key_len = entry.key_offset as u32 - prev_entry.key_offset as u32;
      }

      if idx == header.table_entries - 1 {
        entry.key_len =
          header.data_table_start - (entry.key_offset as u32 + header.key_table_start);
      }

      entries.push(entry);
    }

    Ok(IndexTable { entries })
  }
}

impl Display for IndexTable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "IndexTable:")?;

    for (idx, entry) in self.entries.iter().enumerate() {
      write!(f, "\nKey {idx}:\n")?;
      writeln!(f, "{entry}")?;
    }

    Ok(())
  }
}

#[derive(Clone, Copy, Debug)]
pub struct IndexTableEntry {
  pub key_offset: u16,
  pub key_len: u32,
  pub data_format: Format,
  pub data_len: u32,
  pub data_max_len: u32,
  pub data_offset: u32,
}

impl IndexTableEntry {
  pub fn new<T>(reader: &mut T) -> Result<Self, String>
  where
    T: Read,
  {
    let mut buffer: [u8; 4] = [0; 4];
    reader
      .read_exact(&mut buffer[0..2])
      .map_err(|err| format!("cannot read offset: {err}"))?;
    let key_offset = u16::from_le_bytes(
      buffer[0..2]
        .try_into()
        .map_err(|err| format!("could not convert buffer to a slice of size 2: {err}"))?,
    );

    reader
      .read_exact(&mut buffer[0..2])
      .map_err(|err| format!("cannot read data_format: {err}"))?;
    let data_format: Format = TryInto::<&[u8; 2]>::try_into(&buffer[0..2])
      .map(Format::try_from)
      .map_err(|err| {
        format!(
          "provided value \"{:#X}\" is not a correct format: {err}",
          buffer[1]
        )
      })??;

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read data length start: {err}"))?;
    let data_len = u32::from_le_bytes(buffer);

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read data max length: {err}"))?;
    let data_max_len = u32::from_le_bytes(buffer);

    reader
      .read_exact(&mut buffer)
      .map_err(|err| format!("cannot read data offset: {err}"))?;
    let data_offset = u32::from_le_bytes(buffer);

    Ok(IndexTableEntry {
      key_offset,
      key_len: 0,
      data_format,
      data_len,
      data_max_len,
      data_offset,
    })
  }
}

impl Display for IndexTableEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Key offset: {:#06X}\nData format: {}\nData length: {} bytes [{:#010X}]\nData max length: {} bytes [{:#010X}]\nData offset: {:#010X}",
      self.key_offset,
      self.data_format,
      self.data_len,
      self.data_len,
      self.data_max_len,
      self.data_max_len,
      self.data_offset
    )
  }
}

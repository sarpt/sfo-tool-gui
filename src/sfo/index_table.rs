use std::{
  fmt::Display,
  io::{self, Read, Write},
};

use crate::sfo::{
  format::{self, Format},
  header::Header,
  mapping::DataField,
};

pub struct IndexTable {
  pub entries: Vec<IndexTableEntry>,
}

impl IndexTable {
  pub fn new<T>(reader: &mut T, header: &Header) -> Result<Self, String>
  where
    T: Read,
  {
    let mut entries: Vec<IndexTableEntry> = Vec::new();

    for _ in 0..header.table_entries {
      let entry = IndexTableEntry::new(reader)?;

      entries.push(entry);
    }

    Ok(IndexTable { entries })
  }

  pub fn export<T>(&self, writer: &mut T) -> Result<(), io::Error>
  where
    T: Write,
  {
    for entry in self.entries.iter() {
      entry.export(writer)?;
    }

    Ok(())
  }

  pub fn add(&mut self, idx: usize, key_len: u16, data_field: &DataField) {
    let (key_offset, data_offset) = {
      let shifted_entry = self.entries.get(idx);
      shifted_entry.map_or((0, 0), |entry| (entry.key_offset, entry.data_offset))
    };

    let new_table_entry = IndexTableEntry::for_data_field(data_field, key_offset, data_offset);
    self.entries.insert(idx, new_table_entry);

    for entry in self.entries[idx + 1..].iter_mut() {
      entry.key_offset += key_len;
      entry.data_offset += data_field.len();
    }
  }

  pub fn edit(&mut self, idx: usize, data_field: &DataField) -> Result<(), String> {
    let (prev_len, new_len) = {
      let entry = self.entries.get_mut(idx).ok_or(format!(
        "could not edit entry with index {idx} - no entry with such index"
      ))?;
      let prev_len = entry.data_len;
      match data_field {
        DataField::Utf8String(text) => {
          entry.data_len = (text.len() + 1) as u32;
          entry.data_max_len = (text.len() + 1) as u32;
          entry.data_format = Format::Utf8;
        }
        DataField::U32(_) => {
          entry.data_len = 4;
          entry.data_max_len = 4;
          entry.data_format = Format::U32;
        }
      };
      (prev_len, entry.data_len)
    };

    for entry in self.entries[idx + 1..].iter_mut() {
      entry.data_offset = entry.data_offset - prev_len + new_len;
    }

    Ok(())
  }

  pub fn delete(&mut self, idx: usize, key_len: u16) {
    let removed_entry = self.entries.remove(idx);
    for entry in self.entries[idx..].iter_mut() {
      entry.key_offset -= key_len;
      entry.data_offset -= removed_entry.data_len;
    }
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
      data_format,
      data_len,
      data_max_len,
      data_offset,
    })
  }

  pub fn for_data_field(data_field: &DataField, key_offset: u16, data_offset: u32) -> Self {
    let (data_format, data_len, data_max_len) = match data_field {
      DataField::Utf8String(val) => (
        format::Format::Utf8,
        val.to_string().len() as u32 + 1,
        val.to_string().len() as u32 + 1,
      ),
      DataField::U32(_) => (Format::U32, 4, 4),
    };

    IndexTableEntry {
      key_offset,
      data_format,
      data_len,
      data_max_len,
      data_offset,
    }
  }

  pub fn export<T>(&self, writer: &mut T) -> Result<(), io::Error>
  where
    T: Write,
  {
    writer.write_all(&self.key_offset.to_le_bytes())?;
    writer.write_all(&(Into::<[u8; 2]>::into(self.data_format)))?;
    writer.write_all(&self.data_len.to_le_bytes())?;
    writer.write_all(&self.data_max_len.to_le_bytes())?;
    writer.write_all(&self.data_offset.to_le_bytes())?;

    Ok(())
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

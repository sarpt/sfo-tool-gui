use std::{
  io::{self, Read, Write},
  iter::Enumerate,
};
use thiserror::Error;

use crate::sfo::{
  header::Header,
  index_table::{IndexTable, IndexTableEntry},
  keys::Keys,
  mapping::{DataField, Mapping, MappingIter},
};

pub mod format;
pub mod header;
pub mod index_table;
pub mod keys;
pub mod mapping;

pub struct Sfo {
  pub magic: [u8; 4],
  pub header: Header,
  pub index_table: IndexTable,
  pub entries_mapping: Mapping,
}

const UNCONTAINED_PARAM_SFO_MAGIC: [u8; 4] = [0x00, 0x50, 0x53, 0x46];

#[derive(Error, Debug)]
pub enum SfoParseErr {
  #[error("Could not read entries mappings: {0}")]
  EntriesMappingReadErr(String),
  #[error("Could not read index table: {0}")]
  IndexTableReadErr(String),
  #[error("Could not read header: {0}")]
  HeaderReadErr(String),
  #[error("Could not read magic: {0}")]
  MagicReadErr(String),
  #[error("Provided file doesn't match known .sfo files magic: {0}")]
  UnknownMagic(String),
}

impl Sfo {
  pub fn new<T>(reader: &mut T) -> Result<Self, SfoParseErr>
  where
    T: Read,
  {
    let mut magic: [u8; 4] = [0; 4];
    reader
      .read_exact(&mut magic)
      .map_err(|err| SfoParseErr::MagicReadErr(err.to_string()))?;

    if magic != UNCONTAINED_PARAM_SFO_MAGIC {
      return Err(SfoParseErr::UnknownMagic(format!(
        "Magic {:#04X?} doesn't match any known .sfo file magic",
        &magic
      )));
    }

    let header = Header::new(reader).map_err(SfoParseErr::HeaderReadErr)?;
    let index_table = IndexTable::new(reader, &header).map_err(SfoParseErr::IndexTableReadErr)?;
    let entries_mapping =
      Mapping::new(reader, &index_table).map_err(SfoParseErr::EntriesMappingReadErr)?;

    Ok(Self {
      magic,
      header,
      index_table,
      entries_mapping,
    })
  }

  pub fn export<T>(&self, writer: &mut T) -> Result<(), io::Error>
  where
    T: Write,
  {
    writer.write_all(&self.magic)?;

    self.header.export(writer)?;
    self.index_table.export(writer)?;
    self.entries_mapping.export(writer, &self.index_table)?;

    Ok(())
  }

  pub fn add(&mut self, key: Keys, data_field: DataField) {
    let previous_entry = self.iter().last();
    let (prev_key_len, prev_padding) =
      previous_entry.as_ref().map_or((0, 0), |(prev_key, entry)| {
        let absolute_prev_key_start_offset =
          self.header.key_table_start + entry.index_table_entry.key_offset as u32;
        let absolute_prev_key_end_offset = absolute_prev_key_start_offset + prev_key.len() as u32;
        (
          prev_key.len() as u32,
          self.header.data_table_start - absolute_prev_key_end_offset,
        )
      });
    let key_len_with_padding = key.len() as u32 + 4 - (key.len() as u32 % 4);

    let new_table_entry = IndexTableEntry {
      key_len: key_len_with_padding,
      key_offset: previous_entry
        .as_ref()
        .map_or(0, |(_, prev)| {
          prev.index_table_entry.key_offset as u32 + prev_key_len
        })
        .try_into()
        .unwrap_or_default(),
      data_format: format::Format::Utf8,
      data_len: data_field.to_string().len() as u32 + 1,
      data_max_len: data_field.to_string().len() as u32 + 1,
      data_offset: previous_entry.map_or(0, |(_, prev)| {
        prev.index_table_entry.data_offset + prev.index_table_entry.data_max_len
      }),
    };

    self.header.add_entry(key_len_with_padding - prev_padding);
    if let Some(entry) = self.index_table.entries.iter_mut().last() {
      entry.key_len -= prev_padding;
    };
    self.index_table.entries.push(new_table_entry);
    self.entries_mapping.add(key, data_field);
  }

  pub fn iter<'a>(&'a self) -> SfoEntryIter<'a> {
    let mapping_enumerate = self.entries_mapping.iter().enumerate();

    SfoEntryIter::new(&self.index_table, mapping_enumerate)
  }
}

pub struct SfoEntry<'a> {
  pub data: &'a DataField,
  pub index_table_entry: &'a IndexTableEntry,
}

pub struct SfoEntryIter<'a> {
  index_table: &'a IndexTable,
  mapping_enumerate: Enumerate<MappingIter<'a>>,
}

impl<'a> SfoEntryIter<'a> {
  fn new(index_table: &'a IndexTable, mapping_enumerate: Enumerate<MappingIter<'a>>) -> Self {
    Self {
      index_table,
      mapping_enumerate,
    }
  }
}

impl<'a> Iterator for SfoEntryIter<'a> {
  type Item = (&'a Keys, SfoEntry<'a>);

  fn next(&mut self) -> Option<Self::Item> {
    let (idx, (key, data)) = self.mapping_enumerate.next()?;

    let index_table_entry = self.index_table.entries.get(idx)?;
    let entry = SfoEntry {
      data,
      index_table_entry,
    };
    Some((key, entry))
  }
}

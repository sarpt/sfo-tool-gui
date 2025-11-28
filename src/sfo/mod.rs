use std::{
  io::{self, Read, Seek, Write},
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
  pub padding: usize,
}

const UNCONTAINED_PARAM_SFO_MAGIC: [u8; 4] = [0x00, 0x50, 0x53, 0x46];
const KEY_TABLE_PADDING_ALIGNMENT_BYTES: u32 = 4;

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
    T: Read + Seek,
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
      Mapping::new(reader, &index_table, &header).map_err(SfoParseErr::EntriesMappingReadErr)?;
    let sum_of_keys = entries_mapping.keys_len();
    let padding = (KEY_TABLE_PADDING_ALIGNMENT_BYTES
      - (sum_of_keys as u32 % KEY_TABLE_PADDING_ALIGNMENT_BYTES)) as usize;

    Ok(Self {
      magic,
      header,
      index_table,
      entries_mapping,
      padding,
    })
  }

  pub fn export<T>(&self, writer: &mut T) -> Result<(), io::Error>
  where
    T: Write,
  {
    writer.write_all(&self.magic)?;

    self.header.export(writer)?;
    self.index_table.export(writer)?;
    self
      .entries_mapping
      .export(writer, &self.index_table, self.padding)?;

    Ok(())
  }

  pub fn add(&mut self, key: Keys, data_field: DataField) {
    let sorted_idx = self.entries_mapping.get_sorted_idx(&key);

    let key_len = key.len();
    self
      .index_table
      .add(sorted_idx, key_len as u16, &data_field);
    self.entries_mapping.add(key, data_field);
    let new_padding = self.calculate_padding();
    self
      .header
      .add_entry(key_len as u32, self.padding as u32, new_padding);
    self.padding = new_padding as usize;
  }

  pub fn edit(&mut self, key: &Keys, data_field: DataField) -> Result<(), String> {
    let idx = self.get_idx(key)?;
    self.index_table.edit(idx, &data_field)?;
    self.entries_mapping.edit(key, data_field);
    let new_padding = self.calculate_padding();
    self.header.edit_entry(self.padding as u32, new_padding);
    self.padding = new_padding as usize;
    Ok(())
  }

  pub fn delete(&mut self, key: &Keys) -> Result<(), String> {
    let idx = self.get_idx(key)?;
    self.index_table.delete(idx, key.len() as u16);
    self.entries_mapping.delete(idx, key);
    let new_padding = self.calculate_padding();
    self
      .header
      .delete_entry(key.len() as u32, self.padding as u32, new_padding);
    self.padding = new_padding as usize;
    Ok(())
  }

  fn get_idx(&self, key: &Keys) -> Result<usize, String> {
    self
      .entries_mapping
      .iter()
      .enumerate()
      .find_map(|(idx, (k, _))| match k == key {
        true => Some(idx),
        false => None,
      })
      .ok_or_else(|| format!("could not find idx of key {key}"))
  }

  fn calculate_padding(&self) -> u32 {
    let sum_of_keys = self.entries_mapping.keys_len();
    KEY_TABLE_PADDING_ALIGNMENT_BYTES - (sum_of_keys as u32 % KEY_TABLE_PADDING_ALIGNMENT_BYTES)
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

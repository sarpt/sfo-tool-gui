use std::io::Read;
use thiserror::Error;

use crate::sfo::{header::Header, index_table::IndexTable, mapping::Mapping};

pub mod format;
pub mod header;
pub mod index_table;
pub mod keys;
pub mod mapping;

pub struct Sfo {
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

    println!("Magic: {:#04X?}", &magic);

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
      header,
      index_table,
      entries_mapping,
    })
  }
}

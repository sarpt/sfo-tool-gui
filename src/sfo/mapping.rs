use std::{collections::HashMap, fmt::Display, io::Read, str::FromStr, vec};

use crate::sfo::{format::Format, index_table::IndexTable, keys::Keys};

pub struct Mapping {
  pub entries: HashMap<Keys, DataField>,
}

impl Display for Mapping {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Entries:")?;

    for (key, entry) in self.entries.iter() {
      write!(f, "\n{key}:\n")?;
      writeln!(f, "{entry}")?;
    }

    Ok(())
  }
}

impl Mapping {
  pub fn new<T>(reader: &mut T, index_table: &IndexTable) -> Result<Self, String>
  where
    T: Read,
  {
    let mut keys = Vec::<Keys>::with_capacity(index_table.entries.len());

    for index_table_entry in &index_table.entries {
      let mut buff = vec![0; index_table_entry.key_len as usize];
      reader
        .read_exact(&mut buff)
        .map_err(|err| format!("could not read key: {err}"))?;

      keys.push(key_from_buff(&buff)?);
    }

    let mut entries = HashMap::<Keys, DataField>::new();

    for (idx, index_table_entry) in index_table.entries.iter().enumerate() {
      let key = keys[idx];
      let mut data_buff = vec![0; index_table_entry.data_max_len as usize];
      reader
        .read_exact(&mut data_buff)
        .map_err(|err| format!("could not read data entry with idx {idx} for key {key}: {err}"))?;

      let data: DataField = match index_table_entry.data_format {
        Format::Utf8 | Format::Utf8Special => DataField::Utf8String(
          String::from_utf8(data_buff)
            .map_err(|err| format!("could not map UTF8 string: {err}"))?,
        ),
        Format::U32 => DataField::U32(u32::from_le_bytes(
          data_buff[0..4]
            .try_into()
            .map_err(|err| format!("could not map to u32: {err}"))?,
        )),
      };

      entries.insert(key, data);
    }

    Ok(Mapping { entries })
  }
}

pub enum DataField {
  Utf8String(String),
  U32(u32),
}

impl Display for DataField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DataField::Utf8String(val) => write!(f, "{val}"),
      DataField::U32(val) => write!(f, "{val}"),
    }
  }
}

pub fn key_from_buff(buff: &[u8]) -> Result<Keys, String> {
  let mut nul_range_end: usize = 1;
  for b in buff {
    if *b == 0 {
      break;
    }
    nul_range_end += 1;
  }

  let key_str = str::from_utf8(&buff[0..nul_range_end - 1])
    .map_err(|err| format!("key is not a valid UTF8 string: {err}"))?;
  let key = Keys::from_str(key_str)
    .map_err(|err| format!("unknown key {key_str} with provided: {err}",))?;

  Ok(key)
}

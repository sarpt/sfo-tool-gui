use std::{collections::HashMap, fmt::Display, io::Read, str::FromStr, vec};

use crate::sfo::{format::Format, index_table::IndexTable, keys::Keys};

pub struct Mapping {
  keys_order: Vec<Keys>,
  entries: HashMap<Keys, DataField>,
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
    let mut keys_order = Vec::<Keys>::with_capacity(index_table.entries.len());

    for index_table_entry in &index_table.entries {
      let mut buff = vec![0; index_table_entry.key_len as usize];
      reader
        .read_exact(&mut buff)
        .map_err(|err| format!("could not read key: {err}"))?;

      keys_order.push(key_from_buff(&buff)?);
    }

    let mut entries = HashMap::<Keys, DataField>::new();

    for (idx, index_table_entry) in index_table.entries.iter().enumerate() {
      let key = keys_order[idx].clone();
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

    Ok(Mapping {
      entries,
      keys_order,
    })
  }

  pub fn iter<'a>(&'a self) -> MappingIter<'a> {
    MappingIter::new(self)
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
  let key = Keys::from_str(key_str).unwrap_or_else(|_| Keys::Unknown(key_str.to_owned()));

  Ok(key)
}

pub struct MappingIter<'a> {
  idx: usize,
  mapping: &'a Mapping,
}

impl<'a> MappingIter<'a> {
  fn new(mapping: &'a Mapping) -> Self {
    Self { idx: 0, mapping }
  }
}

impl<'a> Iterator for MappingIter<'a> {
  type Item = (&'a Keys, &'a DataField);

  fn next(&mut self) -> Option<Self::Item> {
    let idx = self.idx;
    self.idx += 1;

    let elem_key = self.mapping.keys_order.get(idx)?;
    let elem = self.mapping.entries.get(elem_key)?;
    Some((elem_key, elem))
  }
}

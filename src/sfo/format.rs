use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Format {
  Utf8Special = 0x00,
  Utf8 = 0x02,
  U32 = 0x04,
}

impl TryFrom<&[u8; 2]> for Format {
  type Error = String;

  fn try_from(bytes: &[u8; 2]) -> Result<Self, Self::Error> {
    if bytes[0] != 0x4 {
      return Err(format!(
        "first byte of format is {:#X}, expected 0x4",
        bytes[0]
      ));
    };

    match bytes[1] {
      0x00 => Ok(Format::Utf8Special),
      0x02 => Ok(Format::Utf8),
      0x04 => Ok(Format::U32),
      _ => Err(format!(
        "second byte of format \"{:#x}\" does not match any known formats",
        bytes[1]
      )),
    }
  }
}

impl Display for Format {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Format::Utf8Special => write!(f, "UTF-8 Non-null terminated [0x0400]"),
      Format::Utf8 => write!(f, "UTF-8 Null terminated [0x0402]"),
      Format::U32 => write!(f, "Unsigned 32-bit integer [0x0404]"),
    }
  }
}

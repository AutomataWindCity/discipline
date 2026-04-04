use crate::x::database::*;

#[derive(Debug, Clone)]
pub enum OptionType {
  None = 0,
  Some = 1,
}

impl OptionType {
  pub fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      0 => {
        Ok(Self::None)
      }
      1 => {
        Ok(Self::Some)
      }
      _ => {
        Err(TextualError::new("action"))
      }
    }
  }

  pub fn to_number(self) -> u8 {
    self as u8
  }
}

impl ScalarRead for OptionType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::TextualError> {
    reader.read_scalar_value().and_then(OptionType::from_number)
  }
}

impl ScalarWrite for OptionType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}
use crate::x::{IsTextualError, TextualError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionVariant {
  None,
  Some,
}

impl OptionVariant {
  pub fn from_number(number: u8, textual_error: &mut impl IsTextualError) -> Result<Self, ()> {
    todo!()
  }
  
  pub fn from_number_or_textual_error(number: u8) -> Result<Self, TextualError> {
    todo!()
  }

  pub fn to_number(self) -> u8 {
    todo!()
  }
}
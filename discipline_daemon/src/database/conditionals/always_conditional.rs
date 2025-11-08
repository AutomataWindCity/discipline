use std::marker::PhantomData;
use crate::x::{AlwaysConditional, TextualError};
use crate::x::database::*;

pub struct AlwaysConditionalSchema {
  __private: PhantomData<()>
}

impl AlwaysConditionalSchema {
  pub fn new() -> Self {
    Self {
      __private: PhantomData
    }
  }
}

impl SerializableCompoundValue for AlwaysConditional {
  type Schema = AlwaysConditionalSchema;

  fn serialize(_value: &Self, _schema: &Self::Schema, _writer: &mut impl CompoundValueWriter) {
    // no operation
  }
}

impl DeserializableCompoundValue for AlwaysConditional {
  type Schema = AlwaysConditionalSchema;

  fn deserialize(_reader: &mut impl CompoundValueReader, _schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(AlwaysConditional::new())
  }
}
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

impl WriteCompoundValue for AlwaysConditional {
  type Schema = AlwaysConditionalSchema;

  fn write(_value: &Self, _schema: &Self::Schema, _writer: &mut impl CompoundValueWriteDestination) {
    // no operation
  }
}

impl ReadCompoundValue for AlwaysConditional {
  type Schema = AlwaysConditionalSchema;

  fn deserialize(_reader: &mut impl CompoundValueReadSource, _schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(AlwaysConditional::new())
  }
}
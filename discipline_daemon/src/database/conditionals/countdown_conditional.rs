use crate::x::{CountdownConditional, TextualError};
use crate::x::database::*;

pub struct CountdownConditionalSchema {
  duration: Key,
  countdown: CountdownSchema,
}

impl CountdownConditionalSchema {
  pub fn new(duration: Key, countdown: CountdownSchema) -> Self {
    Self {
      duration,
      countdown,
    }
  }
}

impl SerializableCompoundValue for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_scalar_value(schema.duration, &value.duration());
    writer.write_compound_value(&schema.countdown, value.countdown());
  }
}

impl DeserializableCompoundValue for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CountdownConditional::from_fields(
      reader.read_scalar_value(schema.duration)?, 
      reader.read_compound_value(&schema.countdown)?,
    ))
  }
}
use crate::x::{TimeConditional, TextualError};
use crate::x::database::*;

pub struct TimeConditionalSchema {
  time_range: TimeRangeSchema,
  weekday_set: Key,
}

impl TimeConditionalSchema {
  pub fn new(
    time_range_from: Key,
    time_range_till: Key,
    weekday_set: Key,
  ) -> Self {
    Self {
      time_range: TimeRangeSchema::new(time_range_from, time_range_till),
      weekday_set,
    }
  }
}

impl SerializableCompoundValue for TimeConditional {
  type Schema = TimeConditionalSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_compound_value(&schema.time_range, &value.time_range());
    writer.write_scalar_value(schema.weekday_set, &value.weekday_set());
  }
}

impl DeserializableCompoundValue for TimeConditional {
  type Schema = TimeConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(TimeConditional::new(
      reader.read_compound_value(&schema.time_range)?, 
      reader.read_scalar_value(schema.weekday_set)?,
    ))
  }
}
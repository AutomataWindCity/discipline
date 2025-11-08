// use crate::x::{TimeRange, TextualError};
// use crate::x::database::*;

// pub struct TimeRangeSchema {
//   from: Key,
//   till: Key,
// }

// impl TimeRangeSchema {
//   pub fn new(
//     from: Key,
//     till: Key,
//   ) -> Self {
//     Self {
//       from,
//       till,
//     }
//   }
// }

// impl SerializableCompoundValue for TimeRange {
//   type Schema = TimeRangeSchema;

//   fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
//     writer.write_scalar_value(schema.from, &value.from());
//     writer.write_scalar_value(schema.till, &value.till());
//   }
// }

// impl DeserializableCompoundValue for TimeRange {
//   type Schema = TimeRangeSchema;

//   fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
//     Ok(TimeRange::from_times(
//       reader.read_scalar_value(schema.from)?, 
//       reader.read_scalar_value(schema.till)?,
//     ))
//   }
// }
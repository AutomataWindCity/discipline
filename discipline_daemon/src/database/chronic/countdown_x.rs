use crate::x::{TextualError, CountdownX};
use crate::x::database::*;

pub struct CountdownXSchema {
  from: Key,
  duration: Key,
}

impl CountdownXSchema {
  pub fn new(from: Key, duration: Key) -> Self {
    Self {
      from,
      duration,
    }
  }
}

// impl SerializableCompoundValue for CountdownX {
//   type Schema = CountdownXSchema;

//   fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
//     writer.write_scalar_value(schema.from, &value.from);
//   }
// }
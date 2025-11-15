use crate::x::{TextualError, Countdown};
use crate::x::database::*;

pub struct CountdownSchema {
  from: Key,
  duration: Key,
}

impl CountdownSchema {
  pub fn new(from: Key, duration: Key) -> Self {
    Self {
      from,
      duration,
    }
  }
}

impl WriteCompoundValue for Countdown {
  type Schema = CountdownSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CountdownValueWriteDestination) {
    writer.write_scalar_value(schema.from, &value.from);
    writer.write_scalar_value(schema.duration, &value.duration);
  }
}

impl ReadCompoundValue for Countdown {
  type Schema = CountdownSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Countdown::construct(
      reader.read_scalar_value(schema.from)?, 
      reader.read_scalar_value(schema.duration)?,
    ))
  }
}

impl WriteUpdates for Countdown {
  type Schema = CountdownSchema;
  
  fn write_updates(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    modifications: &mut CompoundValueWriteDestinationForUpdate,
  ) {
    if modified.from != original.from {
      modifications.write_scalar_value(schema.from, &modified.from);
    }
    if modified.duration != original.duration {
      modifications.write_scalar_value(schema.duration, &modified.duration);
    }
  }
}
use crate::x::{Countdown, DateTime, Duration, TextualError};
use crate::x::database::*;

pub struct CountdownSchema {
  remaining_duration: Key,
  previous_synchronization_time: Key,
}

impl CountdownSchema {
  pub fn new(
    remaining_duration: Key,
    previous_synchronization_time: Key,
  ) -> Self {
    Self {
      remaining_duration,
      previous_synchronization_time,
    }
  }
}

impl SerializableCompoundValue for Countdown {
  type Schema = CountdownSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_scalar_value(schema.remaining_duration, &value.remaining_duration());
    writer.write_scalar_value(schema.previous_synchronization_time, &value.previous_synchronization_time());
  }
}

impl DeserializableCompoundValue for Countdown {
  type Schema = CountdownSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Countdown::from_fields(
      reader.read_scalar_value(schema.remaining_duration)?, 
      reader.read_scalar_value(schema.previous_synchronization_time)?,
    ))
  }
}

impl SerializableCompoundValue for Option<Countdown> {
  type Schema = CountdownSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    match value {
      Some(value) => {
        writer.write_compound_value(schema, value);
      }
      None => {
        writer.write_null(schema.remaining_duration);
        writer.write_null(schema.previous_synchronization_time);
      }
    }
  }
}

impl DeserializableCompoundValue for Option<Countdown> {
  type Schema = CountdownSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(match (
      reader.read_scalar_value(schema.remaining_duration)?, 
      reader.read_scalar_value(schema.previous_synchronization_time)?,
    ) {
      (Some(remaining_duration), Some(previous_synchronization_time)) => {
        Some(Countdown::from_fields(
          remaining_duration,
          previous_synchronization_time,
        ))
      }
      _ => {
        None
      }
    })
  }
}

pub trait CountdownUpdateWriter {
  fn write_remaining_duration(&self, updates: &mut CollectionItemUpdates, new_value: Duration);
  fn write_previous_synchronization_time(&self, updates: &mut CollectionItemUpdates, new_value: DateTime);
}

impl CountdownUpdateWriter for CountdownSchema {
  fn write_remaining_duration(&self, updates: &mut CollectionItemUpdates, new_value: Duration) {
    updates.write_scalar_value(self.remaining_duration, &new_value);
  }

  fn write_previous_synchronization_time(&self, updates: &mut CollectionItemUpdates, new_value: DateTime) {
    updates.write_scalar_value(self.previous_synchronization_time, &new_value);
  }
}
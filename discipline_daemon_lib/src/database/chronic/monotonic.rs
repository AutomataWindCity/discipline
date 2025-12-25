use crate::x::database::*;
use crate::x::{MonotonicInstant, MonotonicClock};

impl WriteScalarValue for MonotonicInstant {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.timestamp());
  }
}

impl ReadScalarValue for MonotonicInstant {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::x::TextualError> {
    reader.read_scalar_value().map(MonotonicInstant::from_timestamp)
  }
}

pub struct MonotonicClockSchema {
  total_elapsed_duration: Key,
  previous_synchronization_time: Key,
  synchronization_interval: Key,
}

impl MonotonicClockSchema {
  pub fn new(
    milliseconds: Key,
    previous_synchronization_time: Key,
    synchronization_interval: Key,
  ) -> Self {
    Self {
      total_elapsed_duration: milliseconds,
      previous_synchronization_time,
      synchronization_interval,
    }
  }
}

impl WriteCompoundValue for MonotonicClock {
  type Schema = MonotonicClockSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.total_elapsed_duration, &value.total_elapsed_duration());
    destination.write_scalar_value(schema.previous_synchronization_time, &value.previous_synchronization_time());
    destination.write_scalar_value(schema.synchronization_interval, &value.synchronization_interval());
  }
}

impl ReadCompoundValue for MonotonicClock {
  type Schema = MonotonicClockSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, crate::x::TextualError> {
    Ok(MonotonicClock::construct(
      source.read_scalar_value(schema.total_elapsed_duration)?,
      source.read_scalar_value(schema.previous_synchronization_time)?,
      source.read_scalar_value(schema.synchronization_interval)?,
    ))
  }
}

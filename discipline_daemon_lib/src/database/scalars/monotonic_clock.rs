use crate::x::database::*;
use crate::x::MonotonicClock;

pub struct MonotonicClockColumnNames {
  pub total_elapsed_duration: ColumnName,
  pub previous_synchronization_realtime: ColumnName,
  pub previous_synchronization_boottime: ColumnName,
  pub maximum_synchronization_interval: ColumnName,
}

pub struct MonotonicClockColumnIndexes {
  pub total_elapsed_duration: ColumnIndex,
  pub previous_synchronization_realtime: ColumnIndex,
  pub previous_synchronization_boottime: ColumnIndex,
  pub maximum_synchronization_interval: ColumnIndex,
}

impl WriteCompoundValue for MonotonicClock {
  type Schema = MonotonicClockColumnNames;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.total_elapsed_duration, &value.total_elapsed_duration());
    destination.write_scalar_value(schema.previous_synchronization_time, &value.previous_synchronization_time());
    destination.write_scalar_value(schema.synchronization_interval, &value.synchronization_interval());
  }
}

impl ReadCompoundValue for MonotonicClock {
  type Schema = MonotonicClockColumnNames;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, crate::x::TextualError> {
    Ok(MonotonicClock::construct(
      source.read_scalar_value(schema.total_elapsed_duration)?,
      source.read_scalar_value(schema.previous_synchronization_time)?,
      source.read_scalar_value(schema.synchronization_interval)?,
    ))
  }
}

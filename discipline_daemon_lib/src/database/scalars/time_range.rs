use crate::x::{TimeRange, TextualError};
use crate::x::database::*;

pub struct TimeRangeSchema {
  from: ColumnName,
  till: ColumnName,
}

impl TimeRangeSchema {
  pub fn new(
    from: ColumnName,
    till: ColumnName,
  ) -> Self {
    Self {
      from,
      till,
    }
  }
}

impl WriteCompoundValue for TimeRange {
  type Schema = TimeRangeSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.from, &value.from());
    writer.write_scalar_value(schema.till, &value.till());
  }
}

impl ReadCompoundValue for TimeRange {
  type Schema = TimeRangeSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(TimeRange::from_times(
      reader.read_scalar_value(schema.from)?, 
      reader.read_scalar_value(schema.till)?,
    ))
  }
}
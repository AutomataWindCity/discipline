use crate::x::{TextualError, Countdown};
use crate::x::database::*;

pub struct CountdownColumnNames {
  from: ColumnName,
  duration: ColumnName,
}

impl CountdownColumnNames {
  pub fn new(from: ColumnName, duration: ColumnName) -> Self {
    Self {
      from,
      duration,
    }
  }
}

pub struct CountdownColumnIndexes {
  from: ColumnIndex,
  duration: ColumnIndex,
}

impl WriteCompoundValue for Countdown {
  type Schema = CountdownColumnNames;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.from, &value.from);
    writer.write_scalar_value(schema.duration, &value.duration);
  }
}

impl ReadCompoundValue for Countdown {
  type Schema = CountdownColumnIndexes;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Countdown::construct(
      reader.read_scalar_value_with_index(schema.from)?, 
      reader.read_scalar_value_with_index(schema.duration)?,
    ))
  }
}

// impl WriteCompoundValueDifferences for Countdown {
//   type Schema = CountdownColumnNames;
  
//   fn write_differences(
//     original: &Self, 
//     modified: &Self,
//     schema: &Self::Schema,
//     modifications: &mut impl CompoundValueWriteDestination,
//   ) {
//     if modified.from != original.from {
//       modifications.write_scalar_value(schema.from, &modified.from);
//     }
//     if modified.duration != original.duration {
//       modifications.write_scalar_value(schema.duration, &modified.duration);
//     }
//   }
// }
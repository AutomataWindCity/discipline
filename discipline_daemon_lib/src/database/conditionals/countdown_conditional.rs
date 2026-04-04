use crate::x::{CountdownConditional, TextualError};
use crate::x::database::*;

pub struct CountdownConditionalSchema {
  countdown: CountdownColumnNames,
}

impl CountdownConditionalSchema {
  pub fn new(
    countdown_from: ColumnName,
    countdown_duration: ColumnName,
  ) -> Self {
    Self {
      countdown: CountdownColumnNames::new(
        countdown_from, 
        countdown_duration,
      ),
    }
  }
}

impl WriteCompoundValue for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_compound_value(&schema.countdown, value.countdown());
  }
}

impl ReadCompoundValue for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CountdownConditional::construct(
      reader.read_compound_value(&schema.countdown)?,
    ))
  }
}

impl WriteCompoundValueDifferences for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn write_differences(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    modifications: &mut impl CompoundValueWriteDestination,
  ) {
    WriteCompoundValueDifferences::write_differences(
      original.countdown(), 
      modified.countdown(), 
      &schema.countdown, 
      modifications,
    );
  }
}
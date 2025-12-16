use crate::x::{CountdownAfterPleaConditional, TextualError};
use crate::x::database::*;

pub struct CountdownAfterPleaConditionalSchema {
  is_activated: Key,
  countdown: CountdownSchema,
}

impl CountdownAfterPleaConditionalSchema {
  pub fn new(
    is_activated: Key,
    countdown_from: Key,
    countdown_duration: Key,
  ) -> Self {
    Self {
      is_activated,
      countdown: CountdownSchema::new(
        countdown_from, 
        countdown_duration,
      ),
    }
  }
}

impl WriteCompoundValue for CountdownAfterPleaConditional {
  type Schema = CountdownAfterPleaConditionalSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.is_activated, &value.is_activated());
    writer.write_compound_value(&schema.countdown, value.countdown());
  }
}

impl ReadCompoundValue for CountdownAfterPleaConditional {
  type Schema = CountdownAfterPleaConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CountdownAfterPleaConditional::construct(
      reader.read_scalar_value(schema.is_activated)?,
      reader.read_compound_value(&schema.countdown)?,
    ))
  }
}

impl WriteCompoundValueDifferences for CountdownAfterPleaConditional {
  type Schema = CountdownAfterPleaConditionalSchema;

  fn write_differences(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    modifications: &mut impl CompoundValueWriteDestination,
  ) {
    if modified.is_activated() != original.is_activated() {
      modifications.write_scalar_value(schema.is_activated, &modified.is_activated());
    }
    
    WriteCompoundValueDifferences::write_differences(
      original.countdown(), 
      modified.countdown(), 
      &schema.countdown, 
      modifications,
    );
  }
}
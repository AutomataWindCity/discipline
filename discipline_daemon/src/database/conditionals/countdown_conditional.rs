use crate::x::{CountdownConditional, TextualError};
use crate::x::database::*;

pub struct CountdownConditionalSchema {
  countdown: CountdownSchema,
}

impl CountdownConditionalSchema {
  pub fn new(
    countdown_from: Key,
    countdown_duration: Key,
  ) -> Self {
    Self {
      countdown: CountdownSchema::new(
        countdown_from, 
        countdown_duration,
      ),
    }
  }
}

impl WriteCompoundValue for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CountdownValueWriteDestination) {
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

impl WriteUpdates for CountdownConditional {
  type Schema = CountdownConditionalSchema;

  fn write_updates(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    modifications: &mut CompoundValueWriteDestinationForUpdate,
  ) {
    WriteUpdates::write_updates(
      original.countdown(), 
      modified.countdown(), 
      &schema.countdown, 
      modifications,
    );
  }
}
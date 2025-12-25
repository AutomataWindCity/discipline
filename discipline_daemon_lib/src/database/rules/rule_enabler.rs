use crate::x::{TextualError, conditionals, rules::*, database::*};

enum RuleEnablerType {
  Countdown,
  CountdownAfterPlea,
}

impl RuleEnablerType {
  const COUNTDOWN_AS_NUMBER: u8 = 0;
  const COUNTDOWN_AFTER_PLEA_AS_NUMBER: u8 = 1;

  fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::COUNTDOWN_AS_NUMBER => {
        Ok(Self::Countdown)
      }
      Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER => {
        Ok(Self::CountdownAfterPlea)
      }
      other => {
        Err(
          TextualError::new(format!("Creating RuleEnablerType from number where valid values are {} (for Countdown) and {} (for CountdownAfterPlea)", Self::COUNTDOWN_AS_NUMBER, Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER))
            .with_message("Number is invalid")
            .with_attachement_display("Number", other)
        )
      }
    }
  }

  fn to_number(&self) -> u8 {
    match self {
      Self::Countdown => Self::COUNTDOWN_AS_NUMBER,
      Self::CountdownAfterPlea => Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER,
    }
  }
}

impl WriteScalarValue for RuleEnablerType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl ReadScalarValue for RuleEnablerType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(RuleEnablerType::from_number)
  }
}

pub struct RuleEnablerSchema {
  enum_type: Key,
  enum_countdown: conditionals::countdown::database::Schema,
  enum_countdown_after_plea: conditionals::countdown_after_plea::database::Schema,
}

impl RuleEnablerSchema {
  pub fn new(
    enum_type: Key,
    enum_data_1: Key,
    enum_data_2: Key,
    enum_data_3: Key,
  ) -> Self {
    Self {
      enum_type,
      enum_countdown: conditionals
        ::countdown
        ::database
        ::Schema::new(
          enum_data_1, 
          enum_data_2,
        )
      ,
      enum_countdown_after_plea: conditionals
        ::countdown_after_plea
        ::database
        ::Schema
        ::new(
          enum_data_1, 
          enum_data_2, 
          enum_data_3,
        )
      ,
    }
  }
}

impl WriteCompoundValue for RuleEnabler {
  type Schema = RuleEnablerSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    match value {
      RuleEnabler::Countdown(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleEnablerType::Countdown);
        writer.write_compound_value(&schema.enum_countdown, inner);
      }
      RuleEnabler::CountdownAfterPlea(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleEnablerType::CountdownAfterPlea);
        writer.write_compound_value(&schema.enum_countdown_after_plea, inner);
      }
    }
  }
}

impl ReadCompoundValue for RuleEnabler {
  type Schema = RuleEnablerSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    let enum_type = reader.read_scalar_value(schema.enum_type)?;
    Ok(match enum_type {
      RuleEnablerType::Countdown => {
        RuleEnabler::Countdown(
          reader.read_compound_value(&schema.enum_countdown)?
        )
      }
      RuleEnablerType::CountdownAfterPlea => {
        RuleEnabler::CountdownAfterPlea(
          reader.read_compound_value(&schema.enum_countdown_after_plea)?
        )
      }
    })
  }
}

impl WriteCompoundValueDifferences for RuleEnabler {
  type Schema = RuleEnablerSchema;

  fn write_differences(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    destination: &mut impl CompoundValueWriteDestination,
  ) {
    match (modified, original) {
      (RuleEnabler::Countdown(modified), RuleEnabler::Countdown(original)) => {
        WriteCompoundValueDifferences::write_differences(
          original, 
          modified, 
          &schema.enum_countdown, 
          destination,
        );
      }
      (RuleEnabler::CountdownAfterPlea(modified), RuleEnabler::CountdownAfterPlea(original)) => {
        WriteCompoundValueDifferences::write_differences(
          original, 
          modified, 
          &schema.enum_countdown_after_plea, 
          destination,
        );
      }
      (RuleEnabler::Countdown(new), _old) => {
        destination.write_scalar_value(schema.enum_type, &RuleEnablerType::Countdown);
        destination.write_compound_value(&schema.enum_countdown, new);
      }
      (RuleEnabler::CountdownAfterPlea(new), _old) => {
        destination.write_scalar_value(schema.enum_type, &RuleEnablerType::CountdownAfterPlea);
        destination.write_compound_value(&schema.enum_countdown_after_plea, new);
      }
    }
  }
}
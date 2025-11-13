use crate::x::TextualError;
use crate::x::rules::*;
use crate::x::database::*;

enum RuleProtectionConditionalType {
  Countdown,
  CountdownAfterPlea,
}

impl RuleProtectionConditionalType {
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
          TextualError::new(format!("Creating RuleProtectionConditionalType from number where valid values are {} (for Countdown) and {} (for CountdownAfterPlea)", Self::COUNTDOWN_AS_NUMBER, Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER))
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

impl SerializableScalarValue for RuleProtectionConditionalType {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl DeserializableScalarValue for RuleProtectionConditionalType {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(RuleProtectionConditionalType::from_number)
  }
}

pub struct RuleProtectionConditionalSchema {
  enum_type: Key,
  enum_countdown: CountdownConditionalSchema,
  enum_countdown_after_plea: CountdownAfterPleaConditionalSchema,
}

impl RuleProtectionConditionalSchema {
  pub fn new(
    enum_type: Key,
    enum_data_1: Key,
    enum_data_2: Key,
    enum_data_3: Key,
    enum_data_4: Key,
  ) -> Self {
    Self {
      enum_type,
      enum_countdown: CountdownConditionalSchema::new(
        enum_data_1, 
        enum_data_2,
        enum_data_3,
      ),
      enum_countdown_after_plea: CountdownAfterPleaConditionalSchema::new(
        enum_data_1, 
        enum_data_2, 
        enum_data_3,
        enum_data_4,
      ),
    }
  }
}

impl SerializableCompoundValue for RuleProtectionConditional {
  type Schema = RuleProtectionConditionalSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    match value {
      RuleProtectionConditional::Countdown(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleProtectionConditionalType::Countdown);
        writer.write_compound_value(&schema.enum_countdown, inner);
      }
      RuleProtectionConditional::CountdownAfterPlea(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleProtectionConditionalType::CountdownAfterPlea);
        writer.write_compound_value(&schema.enum_countdown_after_plea, inner);
      }
    }
  }
}

impl DeserializableCompoundValue for RuleProtectionConditional {
  type Schema = RuleProtectionConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    let enum_type = reader.read_scalar_value(schema.enum_type)?;
    Ok(match enum_type {
      RuleProtectionConditionalType::Countdown => {
        RuleProtectionConditional::Countdown(
          reader.read_compound_value(&schema.enum_countdown)?
        )
      }
      RuleProtectionConditionalType::CountdownAfterPlea => {
        RuleProtectionConditional::CountdownAfterPlea(
          reader.read_compound_value(&schema.enum_countdown_after_plea)?
        )
      }
    })
  }
}
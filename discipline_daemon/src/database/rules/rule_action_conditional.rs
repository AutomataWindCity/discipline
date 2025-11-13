use crate::x::TextualError;
use crate::x::rules::*;
use crate::x::database::*;

enum RuleActionConditionalType {
  Time,
  Always,
}

impl RuleActionConditionalType {
  const TIME: u8 = 0;
  const ALWAYS: u8 = 1;

  fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::TIME => {
        Ok(Self::Time)
      }
      Self::ALWAYS => {
        Ok(Self::Always)
      }
      other => {
        Err(
          TextualError::new(format!("Creating RuleActionConditionalType from number where valid values are {} (for Time) and {} (for Always)", Self::TIME, Self::ALWAYS))
            .with_message("Number is invalid")
            .with_attachement_display("Number", other)
        )
      }
    }
  }

  fn to_number(&self) -> u8 {
    match self {
      Self::Time => Self::TIME,
      Self::Always => Self::ALWAYS,
    }
  }
}

impl SerializableScalarValue for RuleActionConditionalType {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl DeserializableScalarValue for RuleActionConditionalType {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(RuleActionConditionalType::from_number)
  }
}

pub struct RuleActionConditionalSchema {
  enum_type: Key,
  enum_time: TimeConditionalSchema,
  enum_always: AlwaysConditionalSchema,
}

impl RuleActionConditionalSchema {
  pub fn new(
    enum_type: Key,
    enum_data_1: Key,
    enum_data_2: Key,
    enum_data_3: Key,
  ) -> Self {
    Self {
      enum_type,
      enum_time: TimeConditionalSchema::new(
        enum_data_1, 
        enum_data_2, 
        enum_data_3,
      ),
      enum_always: AlwaysConditionalSchema::new(),
    }
  }
}

impl SerializableCompoundValue for RuleActionConditional {
  type Schema = RuleActionConditionalSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    match value {
      RuleActionConditional::Time(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActionConditionalType::Time);;
        writer.write_compound_value(&schema.enum_time, inner);
      }
      RuleActionConditional::Alwaus(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActionConditionalType::Always);
        writer.write_compound_value(&schema.enum_always, inner);
      }
    }
  }
}

impl DeserializableCompoundValue for RuleActionConditional {
  type Schema = RuleActionConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    let enum_type = reader.read_scalar_value(schema.enum_type)?;
    
    Ok(match enum_type {
      RuleActionConditionalType::Time => {
        RuleActionConditional::Time(
          reader.read_compound_value(&schema.enum_time)?
        )
      }
      RuleActionConditionalType::Always => {
        RuleActionConditional::Alwaus(
          reader.read_compound_value(&schema.enum_always)?
        )
      }
    })
  }
}
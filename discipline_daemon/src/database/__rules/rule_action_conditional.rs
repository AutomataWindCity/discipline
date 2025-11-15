use crate::x::TextualError;
use crate::x::rules::*;
use crate::x::database::*;

enum RuleActivatorType {
  Time,
  Always,
}

impl RuleActivatorType {
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
          TextualError::new(format!("Creating RuleActivatorType from number where valid values are {} (for Time) and {} (for Always)", Self::TIME, Self::ALWAYS))
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

impl SerializableScalarValue for RuleActivatorType {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl DeserializableScalarValue for RuleActivatorType {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(RuleActivatorType::from_number)
  }
}

pub struct RuleActivatorSchema {
  enum_type: Key,
  enum_time: TimeConditionalSchema,
  enum_always: AlwaysConditionalSchema,
}

impl RuleActivatorSchema {
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

impl SerializableCompoundValue for RuleActivator {
  type Schema = RuleActivatorSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    match value {
      RuleActivator::Time(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActivatorType::Time);;
        writer.write_compound_value(&schema.enum_time, inner);
      }
      RuleActivator::Alwaus(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActivatorType::Always);
        writer.write_compound_value(&schema.enum_always, inner);
      }
    }
  }
}

impl DeserializableCompoundValue for RuleActivator {
  type Schema = RuleActivatorSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    let enum_type = reader.read_scalar_value(schema.enum_type)?;
    
    Ok(match enum_type {
      RuleActivatorType::Time => {
        RuleActivator::Time(
          reader.read_compound_value(&schema.enum_time)?
        )
      }
      RuleActivatorType::Always => {
        RuleActivator::Alwaus(
          reader.read_compound_value(&schema.enum_always)?
        )
      }
    })
  }
}
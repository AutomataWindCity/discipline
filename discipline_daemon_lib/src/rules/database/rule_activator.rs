use crate::x::{TextualError, conditionals, rules::*, database::*};

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

impl WriteScalarValue for RuleActivatorType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl ReadScalarValue for RuleActivatorType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(RuleActivatorType::from_number)
  }
}

pub struct RuleActivatorSchema {
  enum_type: Key,
  enum_time: conditionals::time::database::Schema,
  enum_always: conditionals::always::database::Schema,
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
      enum_time: conditionals::time::database::Schema::new(
        enum_data_1, 
        enum_data_2, 
        enum_data_3,
      ),
      enum_always: conditionals::always::database::Schema::new(),
    }
  }
}

impl WriteCompoundValue for RuleActivator {
  type Schema = RuleActivatorSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    match value {
      RuleActivator::Time(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActivatorType::Time);
        writer.write_compound_value(&schema.enum_time, inner);
      }
      RuleActivator::Always(inner) => {
        writer.write_scalar_value(schema.enum_type, &RuleActivatorType::Always);
        writer.write_compound_value(&schema.enum_always, inner);
      }
    }
  }
}

impl ReadCompoundValue for RuleActivator {
  type Schema = RuleActivatorSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    let enum_type = reader.read_scalar_value(schema.enum_type)?;
    
    Ok(match enum_type {
      RuleActivatorType::Time => {
        RuleActivator::Time(
          reader.read_compound_value(&schema.enum_time)?
        )
      }
      RuleActivatorType::Always => {
        RuleActivator::Always(
          reader.read_compound_value(&schema.enum_always)?
        )
      }
    })
  }
}
use crate::x::{CountdownAfterPleaConditional, TextualError};
use crate::x::countdown_after_plea_conditional::Status;
use crate::x::database::*;

enum StatusType {
  Activated,
  Deactivating,
  Deactivated,
}

impl StatusType {
  const ACTIVATED_AS_NUMBER: u8 = 0;
  const DEACTIVATING_AS_NUMBER: u8 = 1;
  const DEACTIVATED_AS_NUMBER: u8 = 2;

  fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::ACTIVATED_AS_NUMBER => {
        Ok(Self::Activated)
      }
      Self::DEACTIVATING_AS_NUMBER => {
        Ok(Self::Deactivating)
      }
      Self::DEACTIVATED_AS_NUMBER => {
        Ok(Self::Deactivated)
      }
      other => {
        Err(
          TextualError::new(format!("Creating CountdownAfterPleaConditional StatusType from number where valid values are {} (for Activated), {} (for Deactivating) or {} (for Deactivated)", Self::ACTIVATED_AS_NUMBER, Self::DEACTIVATING_AS_NUMBER, Self::DEACTIVATED_AS_NUMBER))
            .with_message("Number is invalid")
            .with_attachement_display("Number", other)
        )
      }
    }
  }

  fn to_number(&self) -> u8 {
    match self {
      Self::Activated => Self::ACTIVATED_AS_NUMBER,
      Self::Deactivating => Self::DEACTIVATING_AS_NUMBER,
      Self::Deactivated => Self::DEACTIVATED_AS_NUMBER,
    }
  }
}

impl SerializableScalarValue for StatusType {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl DeserializableScalarValue for StatusType {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    reader.read_scalar_value().and_then(StatusType::from_number)
  }
}

pub struct CountdownAfterPleaConditionalSchema {
  duration: Key,
  status_type: Key,
  status_countdown: CountdownSchema,
}

impl CountdownAfterPleaConditionalSchema {
  pub fn new(
    duration: Key,
    status_type: Key,
    status_data_1: Key,
    status_data_2: Key,
  ) -> Self {
    Self {
      duration,
      status_type,
      status_countdown: CountdownSchema::new(
        status_data_1,
        status_data_2,
      ),
    }
  }
}

impl SerializableCompoundValue for CountdownAfterPleaConditional {
  type Schema = CountdownAfterPleaConditionalSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_scalar_value(schema.duration, &value.duration());
    match value.status() {
      Status::Activated => {
        writer.write_scalar_value(schema.status_type, &StatusType::Activated);
      }
      Status::Deactivating { countdown } => {
        writer.write_scalar_value(schema.status_type, &StatusType::Deactivating);
        writer.write_compound_value(&schema.status_countdown, countdown);
      }
      Status::Deactivated => {
        writer.write_scalar_value(schema.status_type, &StatusType::Deactivated);
      }
    }
  }
}

impl DeserializableCompoundValue for CountdownAfterPleaConditional {
  type Schema = CountdownAfterPleaConditionalSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    let duration = reader.read_scalar_value(schema.duration)?;
    
    let status_type = reader.read_scalar_value(schema.status_type)?;
    let status = match status_type {
      StatusType::Activated => {
        Status::Activated
      }
      StatusType::Deactivating => {
        Status::Deactivating { 
          countdown: reader.read_compound_value(&schema.status_countdown)?,
        }
      }
      StatusType::Deactivated => {
        Status::Deactivated
      }
    };
    
    Ok(CountdownAfterPleaConditional::from_fields(
      duration, 
      status,
    ))
  }
}
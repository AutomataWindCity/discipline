use super::*;

pub enum EnablerType {
  CountdownAfterPlea,
}

impl EnablerType {
  const COUNTDOWN_AFTER_PLEA_AS_NUMBER: u8 = 0;

  pub fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER => Ok(Self::CountdownAfterPlea),
      other => Err(
        TextualError::new("Creating VaultEnablerType from variant number").with_message(format!(
          "Unknown variant. Expected {} (for CountdownAfterPlea) but found {}",
          Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER,
          other
        )),
      ),
    }
  }

  pub fn to_number(&self) -> u8 {
    match self {
      Self::CountdownAfterPlea => Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER,
    }
  }
}

impl ScalarWrite for EnablerType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}

impl ScalarRead for EnablerType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader.read_scalar_value().and_then(Self::from_number)
  }
}

pub struct EnablerSchema {
  enum_type: ColumnName,
  enum_countdown_after_plea: conditionals::countdown_after_plea::database::Schema,
}

impl EnablerSchema {
  pub fn new(
    enum_type: ColumnName,
    enum_data_1: ColumnName,
    enum_data_2: ColumnName,
    enum_data_3: ColumnName,
  ) -> Self {
    Self {
      enum_type,
      enum_countdown_after_plea: conditionals
        ::countdown_after_plea
        ::database
        ::Schema
        ::new(
          enum_data_1, 
          enum_data_2, 
          enum_data_3,
        ),
    }
  }
}

impl WriteCompoundValue for VaultEnabler {
  type Schema = EnablerSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    match value {
      VaultEnabler::CountdownAfterPlea(inner) => {
        writer.write_scalar_value(schema.enum_type, &EnablerType::CountdownAfterPlea);
        writer.write_compound_value(&schema.enum_countdown_after_plea, inner);
      }
    }
  }
}

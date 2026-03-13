use crate::x::TextualError;
use crate::x::database::*;
use super::{PerUserInfo, UserId, UserName};

impl WriteScalarValue for UserId {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.inner());
  }
}

impl ReadScalarValue for UserId {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader.read_scalar_value().map(UserId::new)
  }
}

impl WriteScalarValue for UserName {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(value.inner());
  }
}

impl ReadScalarValue for UserName {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader.read_scalar_value().map(UserName::new)
  }
}

pub struct PerUserInfoSchema {
  user_id: Key,
  user_name: Key,
}

impl PerUserInfoSchema {
  pub fn new(
    user_id: Key,
    user_name: Key,
  ) -> Self {
    Self {
      user_id,
      user_name,
    }
  }
}

impl WriteCompoundValue for PerUserInfo {
  type Schema = PerUserInfoSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, &value.user_id);
    writer.write_scalar_value(schema.user_name, &value.user_name);
  }
}

impl ReadCompoundValue for PerUserInfo {
  type Schema = PerUserInfoSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(PerUserInfo {
      user_id: source.read_scalar_value(schema.user_id)?,
      user_name: source.read_scalar_value(schema.user_name)?,
    })
  }
}
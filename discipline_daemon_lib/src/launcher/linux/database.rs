use std::path::PathBuf;
use crate::x::{TextualError, TextualErrorV2};
use crate::x::database::*;
use super::{UserId, UserName, State};

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

pub struct UserProfileSchema {
  user_id: Key,
}

impl UserProfileSchema {
  pub fn new(
    user_id: Key,
  ) -> Self {
    Self {
      user_id,
      user_name,
    }
  }
}

impl WriteCompoundValue for PerUserInfo {
  type Schema = UserProfileSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, &value.user_id);
    writer.write_scalar_value(schema.user_name, &value.user_name);
  }
}

impl ReadCompoundValue for PerUserInfo {
  type Schema = UserProfileSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(PerUserInfo {
      user_id: source.read_scalar_value(schema.user_id)?,
      user_name: source.read_scalar_value(schema.user_name)?,
    })
  }
}

pub struct Database {

}

impl Database {
  pub fn open(
    textual_error: &mut impl TextualErrorV2,
    database_directory: PathBuf, 
  ) -> Result<Self, ()> {
    todo!()
  }

  pub fn load_state(&self, textual_error: &mut impl TextualErrorV2) -> Result<State, ()> {
    todo!()
  }
}

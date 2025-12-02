use crate::x::{Database, ToTextualError, UserName, UuidV4, operating_system, regulation, user_name};
use crate::x::database::*;

impl WriteScalarValue for UserName {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(value.as_string());
  }
}

impl ReadScalarValue for UserName {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::x::TextualError> {
    let user_name = reader.read_scalar_value()?;
    UserName::new(user_name).map_err(|error| {
      error.to_textual_error()
    })
  }
}

static ID: Key = Key::new("id");
static NAME: Key = Key::new("name");
static OPERATING_SYSTEM_INFO_USER_ID: Key = Key::new("operating_system_user_id");
static OPERATING_SYSTEM_INFO_USER_NAME: Key = Key::new("operating_system_user_name");

pub struct CollectionItemSchema {
  id: Key,
  name: Key,
  operating_system_info: operating_system::database::PerUserInfoSchema,
}

impl CollectionItemSchema {
  fn new() -> Self {
    Self {
      id: ID,
      name: NAME,
      operating_system_info: operating_system
        ::database
        ::PerUserInfoSchema
        ::new(
          OPERATING_SYSTEM_INFO_USER_ID, 
          OPERATING_SYSTEM_INFO_USER_NAME,
        )
    }
  }
}

struct CollectionItemWriter<'a> {
  user_id: &'a UuidV4,
  user_name: &'a UserName,
  user_operating_system_info: &'a operating_system::PerUserInfo,
}

impl<'a> CompoundValueWriter for CollectionItemWriter<'a> {
  type Schema = CollectionItemSchema;

  fn write(&self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.id, self.user_id);
    destination.write_scalar_value(schema.name, self.user_name);
    destination.write_compound_value(&schema.operating_system_info, self.user_operating_system_info);
  }
}

pub struct Collection {
  name: String,
  schema: CollectionItemSchema,
}

impl Collection {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: CollectionItemSchema::new(),
    }
  }
}

pub fn write_initialize(
  code: &mut SqlCode,
  collection: &Collection,
) {
  code.write("CREATE TABLE OF NOT EXISTS ");
  code.write(&collection.name);
  code.write(" (");
  code.write_key(ID);
  code.write(" TEXT NOT NULL PRIMARY KEY, ");
  code.write_key(NAME);
  code.write(" TEXT NOT NULL, ");
  // Should this be unique?
  code.write_key(OPERATING_SYSTEM_INFO_USER_ID);
  code.write(" INTEGER NOT NULL, ");
  // Should this be unique?
  code.write_key(OPERATING_SYSTEM_INFO_USER_NAME);
  code.write(" TEXT NOT NULL); ");
}

fn write_add_user(
  code: &mut SqlCode,
  collection: &Collection,
  user_id: &UuidV4,
  user_name: &UserName,
  user_operating_system_info: &operating_system::PerUserInfo,
) {
  code.write("INSERT INTO ");
  code.write(&collection.name);
  code.write_char(' ');
  code.write_compound_value_with_writer_for_insert(
    &collection.schema, 
    &CollectionItemWriter {
      user_id,
      user_name,
      user_operating_system_info,
    }
  );
  code.write_char(';');
}

fn write_delete_user(
  code: &mut SqlCode,
  collection: &Collection,
  user_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write_column_equal_value(ID, user_id);
  code.write_char(';');
}

fn write_change_user_name(
  code: &mut SqlCode,
  collection: &Collection,
  user_id: &UuidV4,
  new_user_name: &UserName,
) {
  code.write("UPDATE ");
  code.write(&collection.name);
  code.write(" SET ");
  code.write_column_equal_value(NAME, new_user_name);
  code.write(" WHERE ");
  code.write_column_equal_value(ID, user_id);
  code.write_char(';');
}

pub enum AddUserError {
  DuplicateId,
  Other,
}

pub async fn add_user(
  database: &Database,
  user_id: &UuidV4,
  user_name: &UserName,
  user_regulation_info: &regulation::PerUserInfo,
  user_operating_system_info: &operating_system::PerUserInfo,
) -> Result<(), AddUserError> {
  let mut code = SqlCode::new();
  write_add_user(
    &mut code, 
    &database.user_collection, 
    user_id, 
    user_name, 
    user_operating_system_info,
  );
  
  let Err(error) = database
    .connection
    .execute(&code)
    .await 
    else
  {
    return Ok(());
  };

  Err(match error {
    DbExecuteError::PrimaryKeyViolation => {
      AddUserError::DuplicateId
    }
    _ => {
      AddUserError::Other
    }
  })
}

pub enum DeleteUserError {
  NoSuchUser,
  Other,
}

pub async fn delete_user(
  database: &Database,
  user_id: &UuidV4,
) -> Result<(), DeleteUserError> {
  let mut code = SqlCode::new();
  write_delete_user(
    &mut code, 
    &database.user_collection, 
    user_id,
  );

  let result = database
    .connection 
    .execute_with_changes(&code)
    .await;

  match result {
    Ok(0) => {
      Err(DeleteUserError::NoSuchUser)
    }
    Ok(1) => {
      Ok(())
    }
    Ok(number) => {
      // TODO: Log this case
      Ok(())
    }
    Err(error) => {
      Err(DeleteUserError::Other)
    }
  }
}

pub enum ChangeUserNameError {
  NoSuchUser,
  Other,
}

pub async fn change_user_name(
  database: &Database,
  user_id: &UuidV4,
  new_user_name: &UserName
) -> Result<(), ChangeUserNameError> {
  let mut code = SqlCode::new();
  write_change_user_name(
    &mut code, 
    &database.user_collection, 
    user_id, 
    new_user_name,
  );

  let result = database 
    .connection 
    .execute_with_changes(&code)
    .await;

  match result {
    Ok(0) => {
      Err(ChangeUserNameError::NoSuchUser)
    }
    Ok(1) => {
      Ok(())
    }
    Ok(number) => {
      // This case shouldn't be reached unless our database schema is corrupted.
      // TODO: Log this case.
      Ok(())
    }
    Err(error) => {
      // TODO: Log this case.
      Err(ChangeUserNameError::Other)
    }
  }
}
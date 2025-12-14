use crate::x::{Database, TextualError, ToTextualError, UserName, UsersSingleton, UuidV4, operating_system};
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

static ID: Key = Key::new("UserId");
static NAME: Key = Key::new("UserName");
static OPERATING_SYSTEM_INFO_USER_ID: Key = Key::new("OperatingSystemUserId");
static OPERATING_SYSTEM_INFO_USER_NAME: Key = Key::new("OperationgSystemUserName");

pub struct CollectionItem {
  pub id: UuidV4,
  pub operating_system_info: operating_system::PerUserInfo,
}

impl ReadCompoundValue for CollectionItem {
  type Schema = CollectionItemSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CollectionItem {
      id: source.read_scalar_value(schema.id)?,
      operating_system_info: source.read_compound_value(&schema.operating_system_info)?,
    }) 
  }
}

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
  code.write("CREATE TABLE IF NOT EXISTS ");
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

fn write_set_user_name(
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

fn write_get_all_users(
  code: &mut SqlCode,
  collection: &Collection,
) {
  code.write("SELECT ");
  code.write_key(ID);
  code.write(", ");
  code.write_key(OPERATING_SYSTEM_INFO_USER_ID);
  code.write(", ");
  code.write_key(OPERATING_SYSTEM_INFO_USER_NAME);
  code.write(" FROM ");
  code.write(&collection.name);
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

  let connection = database.connection.lock().await;

  if let Err(error) = connection.execute_or_textual_error(&code) {
    eprintln!("{error}");
    return Err(DeleteUserError::Other); 
  }

  match connection.changes() {
    0 => {
      Err(DeleteUserError::NoSuchUser)
    }
    1 => {
      Ok(())
    }
    number => {
      let error = TextualError::new("Deleting a user from the database")
        .with_message(format!("Expected the number of effected rows to be 1, but SQLite reported it as {number}. This shouldn't happen. It's either a problem with our SQL code or with SQLite itself."))
        .with_attachement_display("SQL code", code.as_str());

      eprintln!("{error}");

      Err(DeleteUserError::Other)
    }
  }
}

pub enum SetUserNameError {
  NoSuchUser,
  Other,
}

pub async fn set_user_name(
  database: &Database,
  user_id: &UuidV4,
  new_user_name: &UserName
) -> Result<(), SetUserNameError> {
  let mut code = SqlCode::new();
  write_set_user_name(
    &mut code, 
    &database.user_collection, 
    user_id, 
    new_user_name,
  );

  let connection = database.connection.lock().await;

  if let Err(mut error) = connection.execute_or_textual_error(&code) {
    error.change_context("Changing a user's name");
    error.change_context("Modifing the database");
    eprintln!("{error}");
    return Err(SetUserNameError::Other);
  }

  match connection.changes() {
    0 => {
      Err(SetUserNameError::NoSuchUser)
    }
    1 => {
      Ok(())
    }
    number => {
      eprintln!("Setting user name in the database. The number of modified rows is {number}.");
      // This case shouldn't be reached unless our database schema is corrupted.
      // TODO: Log this case.
      Ok(())
    }
  }
}

pub async fn get_all_users<ForEach>(
  database: &Database,
  for_each: ForEach,
) -> Result<(), TextualError>
where 
  ForEach: FnMut(CollectionItem)
{
  let mut code = SqlCode::new();
  write_get_all_users(
    &mut code, 
    &database.user_collection,
  );

  database
    .connection
    .get_multiple(
      &code, 
      &database.user_collection.schema, 
      for_each,
    ).await
}

pub struct SingletonSchema {
  maximum_user_number: Key,
}

impl SingletonSchema {
  pub fn new(maximum_user_number: Key) -> Self {
    Self {
      maximum_user_number,
    }
  }
}

impl ReadCompoundValue for UsersSingleton {
  type Schema = SingletonSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(UsersSingleton::construct(
      source.read_scalar_value(schema.maximum_user_number)?,
    ))
  }
}

impl WriteCompoundValue for UsersSingleton {
  type Schema = SingletonSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.maximum_user_number, &value.get_maximum_user_number());
  }
}
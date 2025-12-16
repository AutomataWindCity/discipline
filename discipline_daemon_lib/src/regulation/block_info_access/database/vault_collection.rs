use super::*;

static USER_ID: Key = Key::new("UserId");
static VAULT_ID: Key = Key::new("VaultId");
static VAULT_NAME: Key = Key::new("VaultName");
static VAULT_PROTECTOR_ENUM_TYPE: Key = Key::new("VaultProtectorEnumType");
static VAULT_PROTECTOR_ENUM_DATA_1: Key = Key::new("VaultProtectorEnumData1");
static VAULT_PROTECTOR_ENUM_DATA_2: Key = Key::new("VaultProtectorEnumData2");
static VAULT_PROTECTOR_ENUM_DATA_3: Key = Key::new("VaultProtectorEnumData3");

pub struct VaultTable {
  name: String,
  schema: VaultSchema,
}

impl VaultTable {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: VaultSchema::new(
        USER_ID,
        VAULT_ID,
        VAULT_NAME,
        VAULT_PROTECTOR_ENUM_TYPE,
        VAULT_PROTECTOR_ENUM_DATA_1,
        VAULT_PROTECTOR_ENUM_DATA_2,
        VAULT_PROTECTOR_ENUM_DATA_3,
      ),
    }
  }
}

pub fn write_initialize_vault_table(
  code: &mut SqlCode,
  table: &VaultTable,
) {
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&table.name);
  code.write(" (");
  code.write_key(VAULT_ID);
  code.write(" TEXT NOT NULL PRIMARY KEY, ");
  code.write_key(USER_ID);
  code.write(" TEXT NOT NULL, ");
  code.write_key(VAULT_NAME);
  code.write(" TEXT NOT NULL, ");
  code.write_key(VAULT_PROTECTOR_ENUM_TYPE);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(VAULT_PROTECTOR_ENUM_DATA_1);
  code.write(", ");
  code.write_key(VAULT_PROTECTOR_ENUM_DATA_2);
  code.write(", ");
  code.write_key(VAULT_PROTECTOR_ENUM_DATA_3);
  code.write(") WITHOUT ROWID;");
}

fn write_insert_vault(
  code: &mut SqlCode,
  table: &VaultTable,
  user_id: &UuidV4,
  vault_id: &UuidV4,
  vault_name: &VaultName,
  vault_protector: &VaultProtector,
) {
  code.write("INSERT INTO ");
  code.write(&table.name);
  code.write_char(' ');
  code.write_compound_value_with_writer_for_insert(
    &table.schema, 
    &VaultWriter {
      user_id,
      vault_id,
      vault_name,
      vault_protector,
    }
  );
  code.write_char(';');
}

fn write_delete_vault(
  code: &mut SqlCode,
  table: &VaultTable,
  vault_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&table.name);
  code.write(" WHERE ");
  code.write_column_equal_value(table.schema.vault_id, vault_id);
  code.write_char(';');
}

fn write_set_vault_name(
  code: &mut SqlCode,
  table: &VaultTable,
  vault_id: &UuidV4,
  new_vault_name: &VaultName,
) {
//   let collection_name = &collection.name;
//   let vault_name_key = collection.schema.vault_name.as_str();
//   let new_vault_name = new_vault_name.to_sqlite_repr();
//   let vault_id = vault_id.to_sqlite_repr();

//   code.write(&format!("UPDATE {collection_name} 
// SET {vault_name_key} = {new_vault_name} 
// WHERE id = {vault_id} 
// RETURNING 
//   CASE 
//     WHEN changes() > 0 
//     THEN 'changed'
//     ELSE 
//       CASE
//         WHEN EXISTS (SELECT 1 FROM {collection_name} WHERE id = {vault_id}))
//           THEN 'changed' 
//           ELSE 'no-such-vault'
//       END

//       CASE 
//         WHEN EXISTS (SELECT 1 FROM {collection_name} WHERE id = {vault_id})) 
//           THEN 'VALUE_ALREADY_SET' 
//           ELSE 'ROW_NOT_EXISTS'
//       END
//     ELSE 'SUCCESS: ' || column_name
//   END as status;
// "));
  code.write("UPDATE ");
  code.write(&table.name);
  code.write(" SET ");
  code.write_column_equal_value(table.schema.vault_name, new_vault_name);
  code.write(" WHERE ");
  code.write_column_equal_value(table.schema.vault_id, vault_id);
  code.write_char(';');
}

pub enum AddVaultError {
  DuplicateId,
  Other,
}

pub async fn insert_vault(
  database: &Database,
  user_id: &UuidV4,
  vault_id: &UuidV4,
  vault_name: &VaultName,
  vault_protector: &VaultProtector,
) -> Result<(), AddVaultError> {
  let mut code = SqlCode::new();
  write_insert_vault(
    &mut code, 
    &database.user_block_info_vault_table, 
    user_id, 
    vault_id, 
    vault_name, 
    vault_protector,
  );

  let connection = database.connection.lock().await;

  let Err(error) = connection.execute(&code) else {
    return Ok(());
  };

  match error {
    DbExecuteError::PrimaryKeyViolation => {
      Err(AddVaultError::DuplicateId)
    }
    DbExecuteError::ForiegnKeyViolation => {
      Err(AddVaultError::Other)
    }
    DbExecuteError::Other(it) => {
      eprintln!("{it}");
      Err(AddVaultError::Other)
    }
  }
}

pub enum DeleteVaultError {
  NoSuchVault,
  Other,
}

pub async fn delete_vault(
  database: &Database,
  vault_id: &UuidV4,
) -> Result<(), DeleteVaultError> {
  let mut code = SqlCode::new();
  write_delete_vault(
    &mut code, 
    &database.user_block_info_vault_table, 
    vault_id,
  );

  let connection = database.connection.lock().await;

  if let Err(error) = connection.execute_or_textual_error(&code) {
    eprintln!("{error}");
    return Err(DeleteVaultError::Other);
  }

  if connection.changes() == 0 {
    return Err(DeleteVaultError::NoSuchVault);
  }

  Ok(())
}

pub enum SetVaultNameError {
  NoSuchVault,
  Noop,
  Other,
}

pub async fn set_vault_name(
  database: &Database,
  vault_id: &UuidV4,
  new_vault_name: &VaultName,
) -> Result<(), SetVaultNameError> {
  let mut code = SqlCode::new();
  write_set_vault_name(
    &mut code, 
    &database.user_block_info_vault_table, 
    vault_id, 
    new_vault_name
  );

  let connection = database.connection.lock().await;

  if let Err(mut error) = connection.execute_or_textual_error(&code) {
    error.change_context("Setting the name of a User Block Info Access Vault");
    eprintln!("{error}");
    return Err(SetVaultNameError::Other);
  }

  // TODO: Create dedicated functions to create errors messages in these common cases.
  if connection.changes() == 0 {
    return Err(SetVaultNameError::NoSuchVault);
  }

  Ok(())
}

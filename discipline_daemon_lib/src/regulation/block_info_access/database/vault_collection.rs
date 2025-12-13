use super::*;

pub struct VaultCollection {
  name: String,
  schema: VaultSchema,
}

impl VaultCollection {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: VaultSchema { 
        user_id: "user_id".into(),
        vault_id: "vault_id".into(),
        vault_name: "vault_name".into(),
        vault_protector: ProtectorSchema::new(
          "protector_enum_type".into(),
          "protector_enum_data_1".into(),
          "protector_enum_data_2".into(),
          "protector_enum_data_3".into(),
        ),
      }
    }
  }
}

fn write_add_vault(
  code: &mut SqlCode,
  collection: &VaultCollection,
  user_id: &UuidV4,
  vault_id: &UuidV4,
  vault_name: &VaultName,
  vault_protector: &VaultProtector,
) {
  code.write("INSERT INTO ");
  code.write(&collection.name);
  code.write_char(' ');
  code.write_compound_value_with_writer_for_insert(
    &collection.schema, 
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
  collection: &VaultCollection,
  vault_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write_column_equal_value(collection.schema.vault_id, vault_id);
  code.write_char(';');
}

fn write_change_vault_name(
  code: &mut SqlCode,
  collection: &VaultCollection,
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
  code.write(&collection.name);
  code.write(" SET ");
  code.write_column_equal_value(collection.schema.vault_name, new_vault_name);
  code.write(" WHERE ");
  code.write_column_equal_value(collection.schema.vault_id, vault_id);
  code.write_char(';');
}

pub enum AddVaultError {
  DuplicateId,
  Other,
}

pub async fn add_vault(
  // location: &Location,
  database: &Database,
  user_id: &UuidV4,
  vault_id: &UuidV4,
  vault_name: &VaultName,
  vault_protector: &VaultProtector,
) -> Result<(), AddVaultError> {
  let mut code = SqlCode::new();
  write_add_vault(
    &mut code, 
    &database.user_block_info_vault_collection, 
    user_id, 
    vault_id, 
    vault_name, 
    vault_protector,
  );

  let Err(error) = database
    .connection
    .execute(&code)
    .await 
    else 
  {
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
    &database.user_block_info_vault_collection, 
    vault_id,
  );

  let changes = match database
    .connection
    .execute_with_changes(&code)
    .await 
  {
    Ok(changes) => {
      changes
    }
    Err(it) => {
      return Err(DeleteVaultError::Other);
    }
  };

  if changes == 0 {
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
  write_change_vault_name(
    &mut code, 
    &database.user_block_info_vault_collection, 
    vault_id, 
    new_vault_name
  );

  let changes = match database
    .connection
    .execute_with_changes(&code)
    .await
  {
    Ok(changes) => {
      changes
    }
    Err(error) => {
      return Err(SetVaultNameError::Other);
    }
  };

  if changes == 0 {
    return Err(SetVaultNameError::NoSuchVault);
  }

  Ok(())
}

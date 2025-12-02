use super::*;

pub struct DatumCollection {
  name: String,
  schema: DatumSchema,
}

impl DatumCollection {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: DatumSchema::new(
        "vault_id".into(), 
        "datum_id".into(), 
        "datum_text".into()
      ),
    }
  }
}

fn write_add_datum(
  code: &mut SqlCode,
  collection: &DatumCollection,
  vault_id: &UuidV4,
  datum_id: &UuidV4,
  datum_text: &Datum,
) {
  code.write("INSERT INTO ");
  code.write(&collection.name);
  code.write_char(' ');
  code.write_compound_value_with_writer_for_insert(
    &collection.schema, 
    &DatumWriter {
      vault_id,
      datum_id,
      datum_text,
    }
  );
  code.write_char(';');
}

fn write_delete_datum(
  code: &mut SqlCode,
  collection: &DatumCollection,
  datum_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write_column_equal_value(collection.schema.datum_id, datum_id);
  code.write_char(';');
}

pub enum AddDatumError {
  DuplicateId,
  NoSuchVault,
  Other,
}

pub async fn add_datum(
  location: &Location,
  database: &Database,
  vault_id: &UuidV4,
  datum_id: &UuidV4,
  datum_text: &Datum,
) -> Result<(), AddDatumError> {
  let mut code = SqlCode::new();
  write_add_datum(
    &mut code, 
    &database.user_block_info_datum_collection, 
    vault_id, 
    datum_id, 
    datum_text,
  );

  database
    .connection
    .execute(&code)
    .await
    .map_err(|error| match error {
      DbExecuteError::PrimaryKeyViolation => {
        AddDatumError::DuplicateId
      }
      DbExecuteError::ForiegnKeyViolation => {
        AddDatumError::NoSuchVault
      }
      DbExecuteError::Other(it) => {
        AddDatumError::Other
      }
    })
}

pub enum DeleteDatumError {
  NoSuchDatum,
  NoSuchVault,
  Other,
}

pub async fn delete_datum(
  location: &Location,
  database: &Database,
  datum_id: &UuidV4,
) -> Result<(), DeleteDatumError> {
  let mut code = SqlCode::new();

  write_delete_datum(
    &mut code, 
    &database.user_block_info_datum_collection, 
    datum_id,
  );

  database
    .connection
    .execute_with_changes(&code)
    .await
    .map_err(|_error| {
      DeleteDatumError::Other
    })
    .and_then(|changes| {
      if changes == 0 {
        Err(DeleteDatumError::NoSuchDatum)
      } else {
        Ok(())
      }
    })
}

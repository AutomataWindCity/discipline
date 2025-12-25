use super::*;

static VAULT_ID: Key = Key::new("VaultId");
static DATUM_ID: Key = Key::new("DatumId");
static DATUM_TEXT: Key = Key::new("DatumText");

pub struct DatumTable {
  name: String,
  schema: DatumSchema,
}

impl DatumTable {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: DatumSchema::new(
        VAULT_ID, 
        DATUM_ID, 
        DATUM_TEXT,
      ),
    }
  }
}

pub fn write_initialize_datum_table(
  code: &mut SqlCode,
  table: &DatumTable,
) {
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&table.name);
  code.write(" (");
  code.write_key(VAULT_ID);
  code.write(" TEXT NOT NULL, ");
  code.write_key(DATUM_ID);
  code.write(" TEXT NOT NULL PRIMARY KEY, ");
  code.write_key(DATUM_TEXT);
  code.write(" TEXT NOT NULL) WITHOUT ROWID;");
}

fn write_add_datum(
  code: &mut SqlCode,
  collection: &DatumTable,
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
  collection: &DatumTable,
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
    .lock()
    .await
    .execute(&code)
    .map_err(|error| match error {
      DbExecuteError::PrimaryKeyViolation => {
        AddDatumError::DuplicateId
      }
      DbExecuteError::ForiegnKeyViolation => {
        AddDatumError::NoSuchVault
      }
      DbExecuteError::Other(it) => {
        let mut error = TextualError::new("User Block Info Access: Adding a new Datum to the database");
        error.add_message("An unexpected SQLite error occured");
        error.add_attachement_display("SQLite error", it);
        error.add_attachement_display("SQLite code", code.as_str());
        eprintln!("{error}");
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
  database: &Database,
  datum_id: &UuidV4,
) -> Result<(), DeleteDatumError> {
  let mut code = SqlCode::new();

  write_delete_datum(
    &mut code, 
    &database.user_block_info_datum_collection, 
    datum_id,
  );

  let connection = database.connection.lock().await;

  if let Err(mut error) = connection.execute_or_textual_error(&code) {
    error.change_context("User Block Info Access: Adding a new Datum to the database");
    return Err(DeleteDatumError::Other);
  }

  match connection.changes() {
    0 => {
      Err(DeleteDatumError::NoSuchDatum)
    }
    1 => {
      Ok(())
    }
    number => {
      let error = TextualError::new("Deleting a User Block Info Access Datum from the database")
        .with_message(format!("Expected the number of effected rows to be 1, but SQLite reported it as {number}. This shouldn't happen. It's either a problem with our SQL code or with SQLite itself."))
        .with_attachement_display("SQL code", code.as_str());

      eprintln!("{error}");

      Err(DeleteDatumError::Other)
    }
  }  
}

use std::{any::type_name, sync::Arc};
use tokio::sync::Mutex;
use rusqlite::types::ValueRef;
use crate::x::TextualError;

pub struct SqlCode {
  value: String
}

impl SqlCode {
  pub fn new() -> Self {
    Self {
      value: String::new(),
    }
  }
  
  pub fn write(&mut self, str: &str) {
    self.value.push_str(str);
  }
  
  pub fn write_key(&mut self, key: Key) {
    self.value.push_str(key.as_str());
  }
  
  pub fn write_char(&mut self, character: char) {
    self.value.push(character);
  }

  pub fn write_scalar_value(&mut self, value: &impl WriteScalarValue) {
    WriteScalarValue::write(value, &mut ScalarValueWriteDestination { code: self });
  }

  pub fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where
    T: WriteCompoundValue
  {
    // WriteScalarValue::write(value, &mut ScalarValueWriteDestination { code: self });
  }

  pub fn as_str(&self) -> &str {
    &self.value
  }
}

pub struct ScalarValueReadSource<'a> {
  value_ref: ValueRef<'a>
}

impl<'a> ScalarValueReadSource<'a> {
  pub fn new(value_ref: ValueRef<'a>) -> Self {
    Self {
      value_ref
    }
  }

  pub fn read_scalar_value<T>(&mut self) -> Result<T, TextualError>
  where 
    T: ReadScalarValue
  {
    T::read(self)
  }
}

fn read_scalar_value<T>(value_ref: ValueRef) -> Result<T, TextualError> 
where 
  T: ReadScalarValue
{
  T::read(&mut ScalarValueReadSource { value_ref })
}

pub trait ReadScalarValue: Sized {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError>;
}

impl ReadScalarValue for u8 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading u8 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading u8 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in u8")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for u16 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading u16 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading u16 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in u16")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for u32 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading u32 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading u32 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in u32")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for u64 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading u64 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading u64 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in u64")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for usize {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading usize from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading usize from ScalarValueReader")
        .with_message("Value is integer but cannot fit in usize")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for i8 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading i8 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading i8 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in i8")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for i16 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading i16 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading i16 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in i16")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for i32 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading i32 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading i32 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in i32")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for i64 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading i64 from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading i64 from ScalarValueReader")
        .with_message("Value is integer but cannot fit in i64")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for isize {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = match reader.value_ref {
      ValueRef::Integer(number) => {
        number
      }
      value => {
        return Err(
          TextualError::new("Reading isize from ScalarValueReader")
            .with_message("Value is not integer")
            .with_attachement_debug("Value", value)
        )
      }
    };

    number.try_into().map_err(|error| {
      TextualError::new("Reading isize from ScalarValueReader")
        .with_message("Value is integer but cannot fit in isize")
        .with_attachement_debug("Value", number)
        .with_attachement_display("Number cast error", error)
    })
  }
}

impl ReadScalarValue for bool {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    match reader.value_ref {
      ValueRef::Integer(0) => {
        Ok(false)
      }
      ValueRef::Integer(1) => {
        Ok(true)
      }
      value => {
        Err(
          TextualError::new("Reading bool from ScalarValueReader")
            .with_message("Value is not integer that is either 0 (for false) or 1 (for true)")
            .with_attachement_debug("Value", value)
        )
      }
    }
  }
}

impl ReadScalarValue for String {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let bytes = match reader.value_ref {
      ValueRef::Text(bytes) => {
        bytes
      }
      value => {
        return Err(
          TextualError::new("Reading String from ScalarValueReader")
            .with_message("Value is not Text")
            .with_attachement_debug("Value", value)
        )
      }
    };

    String::from_utf8(bytes.to_owned()).map_err(|error| {
      TextualError::new("Reading String from ScalarValueReader")
        .with_message("Value is Text, but it's not valid utf-8")
        .with_attachement_debug("Text", error.as_bytes())
        .with_attachement_display("Error", error)
    })
  }
}

impl<T> ReadScalarValue for Option<T>
where 
  T: ReadScalarValue
{
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    if reader.value_ref == ValueRef::Null {
      return Ok(None);
    }
    
    T::read(reader)
      .map(Some)
      .map_err(|error| {
        error
          .with_context(format!("Reading {} from ScalarValueReader", type_name::<Self>()))
          .with_message(format!("Value is not Null and the DeserializableScalarValue implementatio for {} returned an error", type_name::<T>()))
      })
  }
}

pub struct ScalarValueWriteDestination<'a> {
  code: &'a mut SqlCode,
}

impl<'a> ScalarValueWriteDestination<'a> {
  pub fn new(code: &'a mut SqlCode) -> Self {
    Self {
      code
    }
  }

  pub fn write_scalar_value<T>(&mut self, value: &T)
  where 
    T: WriteScalarValue 
  {
    T::write(value, self);
  }
}

fn write_scalar_value<T>(code: &mut SqlCode, value: &T)
where
  T: WriteScalarValue
{
  T::write(value, &mut ScalarValueWriteDestination::new(code));
}

pub trait WriteScalarValue {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination);
}

impl WriteScalarValue for u8 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for u16 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for u32 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for u64 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for usize {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for i8 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for i16 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for i32 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for i64 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for isize {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(value.to_string().as_str());
  }
}

impl WriteScalarValue for bool {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write(if *value {
      "TRUE"
    } else {
      "FALSE"
    });
  }
}

impl WriteScalarValue for String {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.code.write_char('\'');
    for char in value.chars() {
      if char == '\'' {
        writer.code.write("''");
      } else {
        writer.code.write_char(char);
      }
    }
    writer.code.write_char('\'');
  }
}

impl<T> WriteScalarValue for Option<T>
where 
  T: WriteScalarValue
{
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    match value {
      None => {
        writer.code.write("NULL");
      }
      Some(value) => {
        T::write(value, writer);
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Key {
  value: &'static str,
}

impl Key {
  pub const fn new(value: &'static str) -> Self {
    Self {
      value
    }
  }

  pub fn as_str(&self) -> &str {
    self.value
  }
}

pub trait CompoundValueReadSource {
  fn read_scalar_value<T>(&mut self, key: Key) -> Result<T, TextualError>
  where 
    T: ReadScalarValue;

  fn read_compound_value<T>(&mut self, schema: &T::Schema) -> Result<T, TextualError>
  where 
    T: ReadCompoundValue;
}

pub trait ReadCompoundValue: Sized {
  type Schema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError>;
}

pub struct CompoundValueReadSourceForSelect<'a> {
  inner: rusqlite::Row<'a>
}

impl<'a> CompoundValueReadSource for CompoundValueReadSourceForSelect<'a> {
  fn read_scalar_value<T>(&mut self, key: Key) -> Result<T, TextualError>
  where 
    T: ReadScalarValue 
  {
    self
      .inner
      .get_ref(key.as_str())
      .map_err(|error| {
        TextualError::new(format!("Reading a scalar value of type {} from CompoundValueReader", type_name::<T>()))
          .with_message("SQLite returned an error when reading the corresponding column")
          .with_attachement_display("Field name", key.as_str())
          .with_attachement_display("SQLite error", error)
      })
      .and_then(read_scalar_value)
      .map_err(|error| {
        error
          .with_context(format!("Reading a scalar value of type {} from CompoundValueReader", type_name::<T>()))
          .with_message(format!("The DeserializableScalarValue implementation for {} returned an error", type_name::<T>()))
          .with_attachement_display("Field name", key.as_str())
      })
  }

  fn read_compound_value<T>(&mut self, schema: &T::Schema) -> Result<T, TextualError>
  where 
    T: ReadCompoundValue 
  {
    T::deserialize(self, schema).map_err(|error| {
      error
        .with_context(format!("Reading a compound value of type {} from CompoundValueReader", type_name::<T>()))
        .with_message(format!("The DeserializableCompoundValue implementation for {} returned an error", type_name::<T>()))
    })
  }
}

pub trait CompoundValueWriteDestination {
  fn write_null(&mut self, key: Key);

  fn write_scalar_value<T>(&mut self, key: Key, value: &T)
  where 
    T: WriteScalarValue;

  fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where 
    T: WriteCompoundValue;

    
  fn write_compound_value_with_writer<T>(&mut self, schema: &T::Schema, serializer: &T)
  where 
    T: CompoundValueWriter;
}

pub trait WriteCompoundValue {
  type Schema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination);
}

pub trait CompoundValueWriter {
  type Schema;

  fn write(&self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination);
}

pub struct CompoundValueWriteDestinationForInsert {
  keys: SqlCode,
  values: SqlCode,
  did_write_some_values: bool,
}

impl CompoundValueWriteDestinationForInsert {
  pub fn new() -> Self {
    Self {
      keys: SqlCode::new(),
      values: SqlCode::new(),
      did_write_some_values: false,
    }
  }
}

impl CompoundValueWriteDestination for CompoundValueWriteDestinationForInsert {
  fn write_null(&mut self, key: Key) {
    if self.did_write_some_values {
      self.keys.write(", ");
      self.values.write(", ");
    } else {
      self.did_write_some_values = true;
    }

    self.keys.write(key.as_str());
    self.values.write("NULL");
  }

  fn write_scalar_value<T>(&mut self, key: Key, value: &T)
  where 
    T: WriteScalarValue 
  {
    if self.did_write_some_values {
      self.keys.write(", ");
      self.values.write(", ");
    } else {
      self.did_write_some_values = true;
    }

    self.keys.write(key.as_str());
    write_scalar_value(&mut self.values, value);
  }

  fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where 
    T: WriteCompoundValue 
  {
    T::write(value, schema, self);
  }

  fn write_compound_value_with_writer<T>(&mut self, schema: &T::Schema, writer: &T)
  where 
    T: CompoundValueWriter 
  {
    writer.write(schema, self);
  }
}

pub fn write_compound_value_with_writer<T>(
  code: &mut SqlCode,
  schema: &T::Schema,
  writer: &T,
) 
where 
  T: CompoundValueWriter
{
  let mut write_destination = CompoundValueWriteDestinationForInsert::new();
  write_destination.write_compound_value_with_writer(schema, writer);

  // TODO: Panic if no fields were written
  code.write("(");
  code.write(&write_destination.keys.value);
  code.write(") VALUES (");
  code.write(&write_destination.values.value);
  code.write(")");
}

pub struct CompoundValueWriteDestinationForUpdate {
  code: SqlCode,
  did_write_some_updates: bool,
}

impl CompoundValueWriteDestinationForUpdate {
  pub fn new() -> Self {
    Self {
      code: SqlCode::new(),
      did_write_some_updates: false,
    }
  }
}

impl CompoundValueWriteDestinationForUpdate {
  pub fn inner(&self) -> Option<&str> {
    if self.did_write_some_updates {
      Some(self.code.as_str())
    } else {
      None
    }
  }
}

impl CompoundValueWriteDestination for CompoundValueWriteDestinationForUpdate {
  fn write_null(&mut self, key: Key) {
    if self.did_write_some_updates {
      self.code.write(", ");
    } else {
      self.did_write_some_updates = true;
    }

    self.code.write(key.as_str());
    self.code.write(" = ");
    self.code.write("NULL");
  }

  fn write_scalar_value<T>(&mut self, key: Key, value: &T)
  where 
    T: WriteScalarValue 
  {
    if self.did_write_some_updates {
      self.code.write(", ");
    } else {
      self.did_write_some_updates = true;
    }

    self.code.write(key.as_str());
    self.code.write(" = ");
    write_scalar_value(&mut self.code, value);
  }

  fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where 
    T: WriteCompoundValue 
  {
    T::write(value, schema, self);
  }

  fn write_compound_value_with_writer<T>(&mut self, schema: &T::Schema, serializer: &T)
  where 
    T: CompoundValueWriter 
  {
    serializer.write(schema, self);
  }
}

pub struct Connection {
  inner: Arc<Mutex<rusqlite::Connection>>
}

pub enum DbExecuteError {
  Other(rusqlite::Error),
  PrimaryKeyViolation,
  ForiegnKeyViolation,
}

impl Connection {
  pub async fn execute(&self, code: &SqlCode) -> Result<(), DbExecuteError> {
    let Err(error) = self.inner.lock().await.execute_batch(code.as_str()) else {
      return Ok(());
    };
    
    let sqlite_extended_error_code = match error {
      rusqlite::Error::SqliteFailure(error, _) => {
        error.extended_code
      }
      other => {
        return Err(DbExecuteError::Other(other));
      }
    };

    match sqlite_extended_error_code {
      libsqlite3_sys::SQLITE_CONSTRAINT_PRIMARYKEY => {
        Err(DbExecuteError::PrimaryKeyViolation)
      }
      _ => {
        Err(DbExecuteError::Other(error))
      }
    }
  }
}
use std::any::type_name;
use rusqlite::types::ValueRef;
use crate::x::TextualError;

pub struct SqlCode {
  value: String
}

impl SqlCode {
  pub fn write(&mut self, str: &str) {
    self.value.push_str(str);
  }
  pub fn write_char(&mut self, character: char) {
    self.value.push(character);
  }
}

pub struct ScalarValueWrtier<'a> {
  code: &'a mut SqlCode,
}

impl<'a> ScalarValueWrtier<'a> {
  pub fn new(code: &'a mut SqlCode) -> Self {
    Self {
      code
    }
  }

  pub fn write_scalar_value<T>(&mut self, value: &T)
  where 
    T: SerializableScalarValue 
  {
    T::serialize(value, self);
  }
}

pub fn serialize_scalar_value<T>(code: &mut SqlCode, value: &T)
where
  T: SerializableScalarValue
{
  T::serialize(value, &mut ScalarValueWrtier::new(code));
}

pub trait SerializableScalarValue {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier);
}

impl SerializableScalarValue for u8 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for u16 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for u32 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for u64 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for usize {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for i8 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for i16 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for i32 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for i64 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for isize {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(value.to_string().as_str());
  }
}

impl SerializableScalarValue for bool {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.code.write(if *value {
      "TRUE"
    } else {
      "FALSE"
    });
  }
}

impl SerializableScalarValue for String {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
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

impl<T> SerializableScalarValue for Option<T>
where 
  T: SerializableScalarValue
{
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    match value {
      None => {
        writer.code.write("NULL");
      }
      Some(value) => {
        T::serialize(value, writer);
      }
    }
  }
}

pub struct ScalarValueReader<'a> {
  value_ref: ValueRef<'a>
}

impl<'a> ScalarValueReader<'a> {
  pub fn new(value_ref: ValueRef<'a>) -> Self {
    Self {
      value_ref
    }
  }

  pub fn read_scalar_value<T>(&mut self) -> Result<T, TextualError>
  where 
    T: DeserializableScalarValue
  {
    T::deserialize(self)
  }
}

pub fn deserialize_scalar_value<T>(
  value_ref: ValueRef,
) -> Result<T, TextualError> 
where 
  T: DeserializableScalarValue
{
  T::deserialize(&mut ScalarValueReader { value_ref })
}

pub trait DeserializableScalarValue: Sized {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError>;
}

impl DeserializableScalarValue for u8 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for u16 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for u32 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for u64 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for usize {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for i8 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for i16 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for i32 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for i64 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for isize {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for bool {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl DeserializableScalarValue for String {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
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

impl<T> DeserializableScalarValue for Option<T>
where 
  T: DeserializableScalarValue
{
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    if reader.value_ref == ValueRef::Null {
      return Ok(None);
    }
    
    T::deserialize(reader)
      .map(Some)
      .map_err(|error| {
        error
          .with_context(format!("Reading {} from ScalarValueReader", type_name::<Self>()))
          .with_message(format!("Value is not Null and the DeserializableScalarValue implementatio for {} returned an error", type_name::<T>()))
      })
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Key {
  value: &'static str,
}

impl Key {
  pub fn new(value: &'static str) -> Self {
    Self {
      value
    }
  }

  pub fn as_str(&self) -> &str {
    self.value
  }
}

pub trait SerializableCompoundValue {
  type Schema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter);
}

pub trait CompoundValueWriter {
  fn write_null(&mut self, key: Key);

  fn write_scalar_value<T>(&mut self, key: Key, value: &T)
  where 
    T: SerializableScalarValue;

  fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where 
    T: SerializableCompoundValue;
}

pub struct CompoundValueInsertWriter {
  keys: SqlCode,
  values: SqlCode,
  did_write_some_values: bool,
}

impl CompoundValueWriter for CompoundValueInsertWriter {
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
    T: SerializableScalarValue 
  {
    if self.did_write_some_values {
      self.keys.write(", ");
      self.values.write(", ");
    } else {
      self.did_write_some_values = true;
    }

    self.keys.write(key.as_str());
    serialize_scalar_value(&mut self.values, value);
  }

  fn write_compound_value<T>(&mut self, schema: &T::Schema, value: &T)
  where 
    T: SerializableCompoundValue 
  {
    T::serialize(value, schema, self);
  }
}

pub trait CompoundValueReader {
  fn read_scalar_value<T>(&mut self, key: Key) -> Result<T, TextualError>
  where 
    T: DeserializableScalarValue;

  fn read_compound_value<T>(&mut self, schema: &T::Schema) -> Result<T, TextualError>
  where 
    T: DeserializableCompoundValue;
}

pub trait DeserializableCompoundValue: Sized {
  type Schema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError>;
}

pub struct SomeCompoundValueReader<'a> {
  inner: rusqlite::Row<'a>
}

impl<'a> CompoundValueReader for SomeCompoundValueReader<'a> {
  fn read_scalar_value<T>(&mut self, key: Key) -> Result<T, TextualError>
  where 
    T: DeserializableScalarValue 
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
      .and_then(deserialize_scalar_value)
      .map_err(|error| {
        error
          .with_context(format!("Reading a scalar value of type {} from CompoundValueReader", type_name::<T>()))
          .with_message(format!("The DeserializableScalarValue implementation for {} returned an error", type_name::<T>()))
          .with_attachement_display("Field name", key.as_str())
      })
  }

  fn read_compound_value<T>(&mut self, schema: &T::Schema) -> Result<T, TextualError>
  where 
    T: DeserializableCompoundValue 
  {
    T::deserialize(self, schema).map_err(|error| {
      error
        .with_context(format!("Reading a compound value of type {} from CompoundValueReader", type_name::<T>()))
        .with_message(format!("The DeserializableCompoundValue implementation for {} returned an error", type_name::<T>()))
    })
  }
}
use crate::x::{TextualError, ToTextualError, UuidV4};
use crate::database::*;

impl SerializableScalarValue for UuidV4 {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.to_string());
  }
}

impl DeserializableScalarValue for UuidV4 {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let string: String = reader.read_scalar_value()?;

    UuidV4::from_string(&string).map_err(|error| {
      error.to_textual_error()
    })
  }
}
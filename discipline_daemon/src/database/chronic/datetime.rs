use crate::x::{TextualError, DateTime, ToTextualError};
use crate::x::database::{SerializableScalarValue, DeserializableScalarValue, ScalarValueReader, ScalarValueWrtier};

impl SerializableScalarValue for DateTime {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.millisecond_timestamp());
  }
}

impl DeserializableScalarValue for DateTime {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    DateTime::from_millisecond_timestamp(timestamp).map_err(|error| {
      error.to_textual_error()
    })
  }
}
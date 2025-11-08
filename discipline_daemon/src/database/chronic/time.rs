use crate::x::{TextualError, Time, ToTextualError};
use crate::x::database::{SerializableScalarValue, DeserializableScalarValue, ScalarValueReader, ScalarValueWrtier};

impl SerializableScalarValue for Time {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.millisecond_timestamp());
  }
}

impl DeserializableScalarValue for Time {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    Time::from_millisecond_timestamp(timestamp).map_err(|error| {
      error.to_textual_error()
    })
  }
}
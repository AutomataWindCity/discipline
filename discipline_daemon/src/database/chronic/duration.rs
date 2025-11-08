use crate::x::{TextualError, Duration};
use crate::x::database::{SerializableScalarValue, DeserializableScalarValue, ScalarValueReader, ScalarValueWrtier};

impl SerializableScalarValue for Duration {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.milliseconds());
  }
}

impl DeserializableScalarValue for Duration {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    Ok(Duration::from_milliseconds(timestamp))
  }
}
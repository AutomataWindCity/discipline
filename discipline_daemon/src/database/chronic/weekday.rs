use crate::x::{TextualError, Weekday, ToTextualError};
use crate::x::database::{SerializableScalarValue, DeserializableScalarValue, ScalarValueReader, ScalarValueWrtier};

impl SerializableScalarValue for Weekday {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.as_number_from_monday());
  }
}

impl DeserializableScalarValue for Weekday {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let number = reader.read_scalar_value()?;
    Weekday::from_number_from_monday_or_err(number).map_err(|error| {
      error.to_textual_error()
    })
  }
}
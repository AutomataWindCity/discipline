use crate::x::{TextualError, DateTime, ToTextualError};
use crate::x::database::{WriteScalarValue, ReadScalarValue, ScalarValueReadSource, ScalarValueWriteDestination};

impl WriteScalarValue for DateTime {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.millisecond_timestamp());
  }
}

impl ReadScalarValue for DateTime {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    DateTime::from_millisecond_timestamp(timestamp).map_err(|error| {
      error.to_textual_error()
    })
  }
}
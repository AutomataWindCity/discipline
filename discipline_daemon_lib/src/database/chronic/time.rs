use crate::x::{TextualError, Time, ToTextualError};
use crate::x::database::{WriteScalarValue, ReadScalarValue, ScalarValueReadSource, ScalarValueWriteDestination};

impl WriteScalarValue for Time {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.millisecond_timestamp());
  }
}

impl ReadScalarValue for Time {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    Time::from_millisecond_timestamp(timestamp).map_err(|error| {
      error.to_textual_error()
    })
  }
}
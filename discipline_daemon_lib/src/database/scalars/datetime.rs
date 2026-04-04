use crate::x::{TextualError, DateTime, ToTextualError};
use crate::x::database::{ScalarWrite, ScalarRead, ScalarValueReadSource, ScalarValueWriteDestination};

impl ScalarWrite for DateTime {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.as_timestamp());
  }
}

impl ScalarRead for DateTime {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    DateTime::from_timestamp(timestamp).map_err(|error| {
      error.to_textual_error()
    })
  }
}
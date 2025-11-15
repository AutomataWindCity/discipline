use crate::x::{TextualError, Weekday, ToTextualError};
use crate::x::database::{WriteScalarValue, ReadScalarValue, ScalarValueReadSource, ScalarValueWriteDestination};

impl WriteScalarValue for Weekday {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.as_number_from_monday());
  }
}

impl ReadScalarValue for Weekday {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let number = reader.read_scalar_value()?;
    Weekday::from_number_from_monday_or_err(number).map_err(|error| {
      error.to_textual_error()
    })
  }
}
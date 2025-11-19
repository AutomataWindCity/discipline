use crate::x::{TextualError, Duration};
use crate::x::database::{WriteScalarValue, ReadScalarValue, ScalarValueReadSource, ScalarValueWriteDestination};

impl WriteScalarValue for Duration {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.milliseconds());
  }
}

impl ReadScalarValue for Duration {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    Ok(Duration::from_milliseconds(timestamp))
  }
}
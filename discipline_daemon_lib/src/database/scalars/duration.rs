use crate::x::{TextualError, Duration};
use crate::x::database::{ScalarWrite, ScalarRead, ScalarValueReadSource, ScalarValueWriteDestination};

impl ScalarWrite for Duration {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.as_total_milliseconds());
  }
}

impl ScalarRead for Duration {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let timestamp = reader.read_scalar_value()?;
    Ok(Duration::from_milliseconds(timestamp))
  }
}
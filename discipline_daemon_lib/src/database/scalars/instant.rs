use crate::x::database::*;
use crate::x::Instant;

impl ScalarWrite for Instant {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.as_timestamp());
  }
}

impl ScalarRead for Instant {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::x::TextualError> {
    reader.read_scalar_value().map(Instant::from_timestamp)
  }
}
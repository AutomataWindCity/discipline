use crate::x::database::*;
use crate::x::MonotonicInstant;

impl WriteScalarValue for MonotonicInstant {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.timestamp());
  }
}

impl ReadScalarValue for MonotonicInstant {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::x::TextualError> {
    reader.read_scalar_value().map(MonotonicInstant::from_timestamp)
  }
}
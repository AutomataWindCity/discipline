use crate::x::{TextualError, ToTextualError, UuidV4};
use crate::database::*;

impl WriteScalarValue for UuidV4 {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_string());
  }
}

impl ReadScalarValue for UuidV4 {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let string: String = reader.read_scalar_value()?;

    UuidV4::from_string(&string).map_err(|error| {
      error.to_textual_error()
    })
  }
}
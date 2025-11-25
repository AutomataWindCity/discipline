use super::*;

impl WriteScalarValue for Datum {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.string);
  }
}

impl ReadScalarValue for Datum {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(|it| Datum::new(it).map_err(|error| match error {
        CreateDatumFromStringError::LengthViolation { string } => {
          TextualError::new("Creating Datum from String")
            .with_message("Strnig length is invalid")
            .with_attachement_display("Length", string.len())
            .with_attachement_display("String", string)
            .with_attachement_display("Minimum length", Datum::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", Datum::MAXIMUM_LENGTH)
            .with_context("Reading Datum from ScalarValueReadSource. Datum is represented as Text.")
        }
      }))
  }
}
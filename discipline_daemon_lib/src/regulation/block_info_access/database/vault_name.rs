use super::*;

impl WriteScalarValue for VaultName {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.string);
  }
}

impl ReadScalarValue for VaultName {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    reader
      .read_scalar_value()
      .and_then(|it| VaultName::new(it).map_err(|error| match error {
        CreateVaultNameFromStringError::LengthViolation { string } => {
          TextualError::new("Creating VaultName from String")
            .with_message("Strnig length is invalid")
            .with_attachement_display("Length", string.len())
            .with_attachement_display("String", string)
            .with_attachement_display("Minimum length", VaultName::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", VaultName::MAXIMUM_LENGTH)
            .with_context("Reading VaultName from ScalarValueReadSource. VaultName is represented as Text.")
        }
      }))
  }
}
use crate::x::RuleProtectorType;
use crate::x::database::*;

impl ScalarRead for RuleProtectorType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::TextualError> {
    reader.read_scalar_value().and_then(RuleProtectorType::from_number)
  }
}

impl ScalarWrite for RuleProtectorType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}
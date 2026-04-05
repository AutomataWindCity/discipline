use crate::x::RuleEnablerType;
use crate::x::database::*;

impl ScalarRead for RuleEnablerType {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, crate::TextualError> {
    reader.read_scalar_value().and_then(RuleEnablerType::from_number)
  }
}

impl ScalarWrite for RuleEnablerType {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.to_number());
  }
}
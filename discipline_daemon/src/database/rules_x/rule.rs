use crate::x::TextualError;
use crate::x::rules::rules_x::*;
use crate::x::database::*;

pub struct RuleSchema {
  action_conditional: RuleActionConditionalSchema,
  protection_conditional: RuleProtectionConditionalSchema,
  is_activated: Key,
}

impl RuleSchema {
  pub fn new(
    action_conditional_enum_type: Key,
    action_conditional_enum_data_1: Key,
    action_conditional_enum_data_2: Key,
    action_conditional_enum_data_3: Key,
    protection_conditional_enum_type: Key,
    protection_conditional_enum_data_1: Key,
    protection_conditional_enum_data_2: Key,
    protection_conditional_enum_data_3: Key,
    protection_conditional_enum_data_4: Key,
    is_activated: Key,
  ) -> Self {
    Self {
      action_conditional: RuleActionConditionalSchema::new(
        action_conditional_enum_type, 
        action_conditional_enum_data_1,
        action_conditional_enum_data_2,
        action_conditional_enum_data_3,
      ),
      protection_conditional: RuleProtectionConditionalSchema::new(
        protection_conditional_enum_type, 
        protection_conditional_enum_data_1, 
        protection_conditional_enum_data_2, 
        protection_conditional_enum_data_3, 
        protection_conditional_enum_data_4,
      ),
      is_activated,
    }
  }
}

impl SerializableCompoundValue for RuleX {
  type Schema = RuleSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_scalar_value(schema.is_activated, &value.is_activated());
    writer.write_compound_value(&schema.action_conditional, value.action_conditional());
    writer.write_compound_value(&schema.protection_conditional, value.protection_conditional());
  }
}

impl DeserializableCompoundValue for RuleX {
  type Schema = RuleSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(RuleX::construct(
      reader.read_scalar_value(schema.is_activated)?,
      reader.read_compound_value(&schema.action_conditional)?,
      reader.read_compound_value(&schema.protection_conditional)?,
    ))
  }
}

pub struct Tory {
  connection: C
}
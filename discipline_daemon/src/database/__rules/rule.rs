use crate::x::TextualError;
use crate::x::rules::*;
use crate::x::database::*;

pub struct RuleSchema {
  activator: RuleActivatorSchema,
  enabler: RuleEnablerSchema,
  is_activated: Key,
}

impl RuleSchema {
  pub fn new(
    activator_enum_type: Key,
    activator_enum_data_1: Key,
    activator_enum_data_2: Key,
    activator_enum_data_3: Key,
    enabler_enum_type: Key,
    enabler_enum_data_1: Key,
    enabler_enum_data_2: Key,
    enabler_enum_data_3: Key,
    enabler_enum_data_4: Key,
    is_activated: Key,
  ) -> Self {
    Self {
      activator: RuleActivatorSchema::new(
        activator_enum_type, 
        activator_enum_data_1,
        activator_enum_data_2,
        activator_enum_data_3,
      ),
      enabler: RuleEnablerSchema::new(
        enabler_enum_type, 
        enabler_enum_data_1, 
        enabler_enum_data_2, 
        enabler_enum_data_3, 
        enabler_enum_data_4,
      ),
      is_activated,
    }
  }
}

impl SerializableCompoundValue for Rule {
  type Schema = RuleSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    writer.write_scalar_value(schema.is_activated, &value.is_activated());
    writer.write_compound_value(&schema.activator, value.activator());
    writer.write_compound_value(&schema.enabler, value.enabler());
  }
}

impl DeserializableCompoundValue for Rule {
  type Schema = RuleSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Rule::construct(
      reader.read_scalar_value(schema.is_activated)?,
      reader.read_compound_value(&schema.activator)?,
      reader.read_compound_value(&schema.enabler)?,
    ))
  }
}
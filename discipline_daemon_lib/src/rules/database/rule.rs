use crate::x::{TextualError, rules::*, database::*};

pub struct RuleSchema {
  pub enabler: RuleEnablerSchema,
  pub activator: RuleActivatorSchema,
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
      ),
    }
  }
}

impl WriteCompoundValue for Rule {
  type Schema = RuleSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_compound_value(&schema.activator, value.activator());
    writer.write_compound_value(&schema.enabler, value.enabler());
  }
}

impl ReadCompoundValue for Rule {
  type Schema = RuleSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Rule::construct(
      reader.read_compound_value(&schema.activator)?,
      reader.read_compound_value(&schema.enabler)?,
    ))
  }
}

impl WriteCompoundValueDifferences for Rule {
  type Schema = RuleSchema;

  fn write_differences(
    original: &Self, 
    modified: &Self,
    schema: &Self::Schema,
    modifications: &mut impl CompoundValueWriteDestination,
  ) {
    WriteCompoundValueDifferences::write_differences(
      original.enabler(),
      modified.enabler(),
      &schema.enabler,
      modifications,
    );
  }
}
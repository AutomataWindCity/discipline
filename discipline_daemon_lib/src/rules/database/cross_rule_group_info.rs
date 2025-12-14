use crate::x::database::*;
use crate::x::rules::*;

pub struct CrossRuleGroupInfoSchema {
  rule_number: Key,
  maximum_rule_number: Key,
}

impl CrossRuleGroupInfoSchema {
  pub fn new(
    rule_number: Key,
    maximum_rule_number: Key,
  ) -> Self {
    Self {
      rule_number,
      maximum_rule_number,
    }
  }
}

impl WriteCompoundValue for RulesSingleton {
  type Schema = CrossRuleGroupInfoSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.rule_number, &value.rule_number);
    destination.write_scalar_value(schema.maximum_rule_number, &value.maximum_rule_number);
  }
}

impl ReadCompoundValue for RulesSingleton {
  type Schema = CrossRuleGroupInfoSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, crate::x::TextualError> {
    Ok(RulesSingleton {
      rule_number: source.read_scalar_value(schema.rule_number)?,
      maximum_rule_number: source.read_scalar_value(schema.maximum_rule_number)?,
    })
  }
}
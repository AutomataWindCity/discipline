use serde::{Deserialize, Serialize};
use crate::x::rules;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
  pub rules: rules::RuleGroup,
}

impl Regulation {
  pub fn new() -> Self {
    Self {
      rules: rules::RuleGroup::new(),
    }
  }

  pub fn construct(rules: rules::RuleGroup) -> Self {
    Self {
      rules,
    }
  }

  pub fn rules(&self) -> &rules::RuleGroup {
    &self.rules
  }

  pub fn rules_mut(&mut self) -> &mut rules::RuleGroup {
    &mut self.rules
  }
}
pub mod rules_x;


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, DateTime, Time, TimeConditional, UuidV4, Weekday};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActionConditional {
  Time(TimeConditional),
  Alwaus(AlwaysConditional),
}

impl RuleActionConditional {
  pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
    match self {
      RuleActionConditional::Time(inner) => inner.evaulate(time, weekday),
      RuleActionConditional::Alwaus(inner) => inner.evaulate(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleProtectionConditional {
  Countdown(CountdownConditional),
  CountdownAfterPlea(CountdownAfterPleaConditional),
}

impl RuleProtectionConditional {
  pub fn evaluate(&self) -> bool {
    match self {
      RuleProtectionConditional::Countdown(inner) => inner.is_activated(),
      RuleProtectionConditional::CountdownAfterPlea(inner) => inner.is_activated_or_deactivating(),
    }
  }

  pub fn synchronize(&mut self, now: DateTime) {
    match self {
      RuleProtectionConditional::Countdown(inner) => inner.synchronize(now),
      RuleProtectionConditional::CountdownAfterPlea(inner) => inner.synchronize(now),
    }
  }

  pub fn activate(&mut self, now: DateTime) {
    match self {
      RuleProtectionConditional::Countdown(inner) => inner.activate(now),
      RuleProtectionConditional::CountdownAfterPlea(inner) => inner.activate(),
    }
  }

  pub fn deactivate(&mut self, now: DateTime) {
    match self {
      RuleProtectionConditional::Countdown(_inner) => {
        // noop
      }
      RuleProtectionConditional::CountdownAfterPlea(inner) => {
        inner.deactivate(now);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
  action_conditional: RuleActionConditional,
  protection_conditional: RuleProtectionConditional,
  is_activated: bool,
}

impl Rule {
  pub fn new(
    action_conditional: RuleActionConditional,
    protection_conditional: RuleProtectionConditional,
  ) -> Self {
    Self {
      action_conditional,
      protection_conditional,
      is_activated: false,
    }
  }

  pub fn construct(
    is_activated: bool,
    action_conditional: RuleActionConditional,
    protection_conditional: RuleProtectionConditional,
  ) -> Self {
    Self {
      is_activated,
      action_conditional,
      protection_conditional,
    }
  }
  
  pub fn action_conditional(&self) -> &RuleActionConditional {
    &self.action_conditional
  }

  pub fn protection_conditional(&self) -> &RuleProtectionConditional {
    &self.protection_conditional
  }

  pub fn is_activated(&self) -> bool {
    self.is_activated
  }

  pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
    self.is_activated
    && 
    self.action_conditional.evaluate(time, weekday)
  }

  pub fn is_protected(&self) -> bool {
    self.protection_conditional.evaluate()
  }
}

pub struct RuleGroup {
  rules: HashMap<UuidV4, Rule>,
  maximum_rule_number: usize,
}

impl RuleGroup {
  pub fn new(maximum_rule_number: usize) -> Self {
    Self {
      rules: HashMap::new(),
      maximum_rule_number,
    }
  }
}

pub mod operations {
  use crate::x::{RuleProtectionConditional, UuidV4, countdown_after_plea_conditional, countdown_conditional};

  // DaemonUsersRegulationBlockInternetAddRule
  // DaemonUsersRegulationBlockInternetDeleteRule
  // DaemonUsersRegulationBlockInternetActivateRule
  // DaemonUsersRegulationBlockInternetDeactivateRule

  // ProtectionConditionalTypeMismatch
  // AlreadyActivated

  pub enum RuleProtectionConditionalActivate {
    Countdown(countdown_conditional::operations::Activate),
    CountdownAfterPlea(countdown_after_plea_conditional::operations::Activate),
  }

  pub enum RuleProtectionConditionalActivateReturn {
    TypeMismatch,
    Countdown(countdown_conditional::operations::ActivateReturn),
    CountdownAfterPlea(countdown_after_plea_conditional::operations::ActivateReturn),
  }
  
  impl RuleProtectionConditionalActivate {
    pub fn execute(
      self,
      conditional: &mut RuleProtectionConditional,
    ) -> RuleProtectionConditionalActivateReturn {
      match (self, conditional) {
        (
          RuleProtectionConditionalActivate::Countdown(operation), 
          RuleProtectionConditional::Countdown(conditional)
        ) => {
          RuleProtectionConditionalActivateReturn::Countdown(
            operation.execute(conditional)
          )
        }
        (
          RuleProtectionConditionalActivate::CountdownAfterPlea(operation), 
          RuleProtectionConditional::CountdownAfterPlea(conditional)
        ) => {
          RuleProtectionConditionalActivateReturn::CountdownAfterPlea(
            operation.execute(conditional)
          )
        }
        _ => {
          RuleProtectionConditionalActivateReturn::TypeMismatch
        }
      }
    }
  }

  pub struct Activate {

  }
}
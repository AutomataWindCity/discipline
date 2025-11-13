use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownAfterPleaConditionalX, CountdownConditional, CountdownConditionalX, DateTime, InstantX, Time, TimeConditional, UuidV4, Weekday, always_conditional, countdown_after_plea_conditional_x, countdown_conditional_x, time_conditional, time_range, time_x};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActionConditionalX {
  Time(TimeConditional),
  Alwaus(AlwaysConditional),
}

impl RuleActionConditionalX {
  pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
    match self {
      RuleActionConditionalX::Time(inner) => inner.evaulate(time, weekday),
      RuleActionConditionalX::Alwaus(inner) => inner.evaulate(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleProtectionConditionalX {
  Countdown(CountdownConditionalX),
  CountdownAfterPlea(CountdownAfterPleaConditionalX),
}

impl RuleProtectionConditionalX {
  pub fn evaluate(&self, now: InstantX) -> bool {
    match self {
      RuleProtectionConditionalX::Countdown(inner) => {
        inner.is_activated(now)
      }
      RuleProtectionConditionalX::CountdownAfterPlea(inner) => {
        inner.is_activated_or_deactivating(now)
      }
    }
  }

  pub fn activate(&mut self, now: InstantX) {
    match self {
      RuleProtectionConditionalX::Countdown(inner) => {
        inner.activate(now);
      }
      RuleProtectionConditionalX::CountdownAfterPlea(inner) => {
        inner.activate()
      }
    }
  }

  pub fn deactivate(&mut self, now: time_x::InstantX) {
    match self {
      RuleProtectionConditionalX::Countdown(_inner) => {
        // noop
      }
      RuleProtectionConditionalX::CountdownAfterPlea(inner) => {
        inner.deactivate(now);
      }
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

#[derive(Debug, Clone)]
pub struct RuleX {
  is_activated: bool,
  action_conditional: RuleActionConditionalX,
  protection_conditional: RuleProtectionConditionalX,
}

impl RuleX {
  pub fn new(
    action_conditional: RuleActionConditionalX,
    protection_conditional: RuleProtectionConditionalX,
  ) -> Self {
    Self {
      is_activated: false,
      action_conditional,
      protection_conditional,
    }
  }

  pub fn construct(
    is_activated: bool,
    action_conditional: RuleActionConditionalX,
    protection_conditional: RuleProtectionConditionalX,
  ) -> Self {
    Self {
      is_activated,
      action_conditional,
      protection_conditional,
    }
  }
  
  pub fn action_conditional(&self) -> &RuleActionConditionalX {
    &self.action_conditional
  }

  pub fn protection_conditional(&self) -> &RuleProtectionConditionalX {
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

  pub fn is_protected(&self, now: InstantX) -> bool {
    self.protection_conditional.evaluate(now)
  }

  pub fn activate(&mut self, now: InstantX) {
    self.is_activated = true;
    self.protection_conditional.activate(now);
  }
}

pub struct RuleGroupX {
  rules: HashMap<UuidV4, RuleX>,
  maximum_rule_number: usize,
}

impl RuleGroupX {
  pub fn new(maximum_rule_number: usize) -> Self {
    Self {
      rules: HashMap::new(),
      maximum_rule_number,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActionConditionalCreator {
  Time(time_conditional::Creator),
  Always(always_conditional::Creator)
}

impl RuleActionConditionalCreator {
  pub fn create(self) -> RuleActionConditionalX {
    match self {
      Self::Time(creator) => {
        RuleActionConditionalX::Time(creator.create())
      }
      Self::Always(creator) => {
        RuleActionConditionalX::Alwaus(creator.create())
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleProtectionConditionalCreator {
  Countdown(countdown_conditional_x::Creator),
  CountdownAfterPlea(countdown_after_plea_conditional_x::Creator)
}

impl RuleProtectionConditionalCreator {
  pub fn create(self) -> RuleProtectionConditionalX {
    match self {
      Self::Countdown(creator) => {
        RuleProtectionConditionalX::Countdown(creator.create())
      }
      Self::CountdownAfterPlea(creator) => {
        RuleProtectionConditionalX::CountdownAfterPlea(creator.create())
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreator {
  id: Option<UuidV4>,
  action_conditional: RuleActionConditionalCreator,
  protection_conditional: RuleProtectionConditionalCreator,
}

pub enum AddRuleError {
  BadUuidV4TryAgain,
  ReachedMaximumRulesAllowedDeleteSomeAndTryAgain,
}

pub enum DeleteRuleSuccess {
  NoSuchRule,
  Success,
}

pub enum DeleteRuleError {
  RuleIsProtected,
}

impl RuleGroupX {
  pub fn add_rule_given_rule_creator(
    &mut self, 
    rule_creator: RuleCreator,
  ) -> Result<(), AddRuleError> {
    if self.rules.len() >= self.maximum_rule_number {
      return Err(AddRuleError::ReachedMaximumRulesAllowedDeleteSomeAndTryAgain);
    }

    let id = rule_creator.id.unwrap_or_else(UuidV4::generate);
    if self.rules.contains_key(&id) {
      return Err(AddRuleError::BadUuidV4TryAgain);
    }

    let rule = RuleX::new(
      rule_creator.action_conditional.create(),
      rule_creator.protection_conditional.create(),
    );

    self.rules.insert(id, rule);
    Ok(())
  }

  pub fn delete_rule_given_rule_id(
    &mut self, 
    now: InstantX,
    rule_id: &UuidV4,
  ) -> Result<DeleteRuleSuccess, DeleteRuleError> {
    let Some(rule) = self.rules.get_mut(rule_id) else {
      return Ok(DeleteRuleSuccess::NoSuchRule);
    };

    if rule.is_protected(now) {
      return Err(DeleteRuleError::RuleIsProtected);
    }

    self.rules.remove(rule_id);
    Ok(DeleteRuleSuccess::Success)
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
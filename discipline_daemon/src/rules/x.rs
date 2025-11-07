use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, DateTime, Time, TimeConditional, UuidV4, Weekday};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActivator {
  Time(TimeConditional),
  Alwaus(AlwaysConditional),
}

impl RuleActivator {
  pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
    match self {
      RuleActivator::Time(inner) => inner.evaulate(time, weekday),
      RuleActivator::Alwaus(inner) => inner.evaulate(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleProtector {
  Countdown(CountdownConditional),
  CountdownAfterPlea(CountdownAfterPleaConditional),
}

impl RuleProtector {
  pub fn is_effective(&self) -> bool {
    match self {
      RuleProtector::Countdown(inner) => inner.evaluate(),
      RuleProtector::CountdownAfterPlea(inner) => inner.evaluate(),
    }
  }

  pub fn synchronize(&mut self, now: DateTime) {
    match self {
      RuleProtector::Countdown(inner) => inner.synchronize(now),
      RuleProtector::CountdownAfterPlea(inner) => inner.synchronize(now),
    }
  }

  pub fn activate(&mut self, now: DateTime) {
    match self {
      RuleProtector::Countdown(inner) => inner.activate(now),
      RuleProtector::CountdownAfterPlea(inner) => inner.activate(),
    }
  }

  pub fn deactivate(&mut self, now: DateTime) {
    match self {
      RuleProtector::Countdown(_inner) => {
        // noop
      }
      RuleProtector::CountdownAfterPlea(inner) => {
        inner.deactivate(now);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
  activator: RuleActivator,
  protector: RuleProtector,
}

impl Rule {
  pub fn new(
    activator: RuleActivator,
    protector: RuleProtector,
  ) -> Self {
    Self {
      activator,
      protector,
    }
  }

  pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
    self.protector.is_effective() && self.activator.is_effective(time, weekday)
  }

  pub fn is_protected(&self) -> bool {
    self.protector.is_effective()
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
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{CountdownAfterPleaConditional, CountdownConditional, Duration, Instant, Time, TimeRange, UuidV4};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnabler {
  Countdown(CountdownConditional),
  CountdownAfterPlea(CountdownAfterPleaConditional),
}

impl RuleEnabler {
  pub fn is_rule_enabled(&self, time: Instant) -> bool {
    match self {
      Self::Countdown(enabler) => {
        enabler.is_active(time)
      }
      Self::CountdownAfterPlea(enabler) => {
        enabler.is_activate_or_deactivating(time)
      }
    }
  }

  pub fn enable(&mut self, now: Instant) {
    match self {
      Self::Countdown(enabler) => {
        enabler.activate(now);
      }
      Self::CountdownAfterPlea(enabler) => {
        enabler.activate();
      }
    }
  }

  pub fn disable(&mut self, now: Instant) {
    match self {
      Self::Countdown(enabler) => {
        // TODO
      }
      Self::CountdownAfterPlea(enabler) => {
        enabler.deactivate(now);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRangeRule {
  pub enabler: RuleEnabler,
  pub condition: TimeRange,
}

impl TimeRangeRule {
  pub fn is_enabled(&self, time: Instant) -> bool {
    self.enabler.is_rule_enabled(time)
  }

  pub fn is_activated(
    &self, 
    time: Time,
    instant: Instant,
  ) -> bool {
    self.enabler.is_rule_enabled(instant)
    &&
    self.condition.contains(time)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeRangeRules {
  rules: HashMap<UuidV4, TimeRangeRule>,
}

impl TimeRangeRules {
  pub fn new() -> Self {
    Self {
      rules: HashMap::new(),
    }
  }

  pub fn are_some_active(
    &self,
    time: Time,
    instant: Instant,
  ) -> bool {
    self.rules.values().any(|rule| {
      rule.is_activated(time, instant)
    })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlwaysRule {
  pub enabler: RuleEnabler,
}

impl AlwaysRule {
  pub fn create(enabler: RuleEnabler) -> AlwaysRule {
    AlwaysRule { enabler }
  }
  
  pub fn is_enabled(&self, now: Instant) -> bool {
    self.enabler.is_rule_enabled(now)
  }

  pub fn is_active(&self, now: Instant) -> bool {
    self.enabler.is_rule_enabled(now)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlwaysRules {
  pub rules: HashMap<UuidV4, AlwaysRule>,
}

impl AlwaysRules {
  pub fn new() -> Self {
    Self {
      rules: HashMap::new(),
    }
  }

  pub fn are_some_active(&self, now: Instant) -> bool {
    self.rules.values().any(|rule| {
      rule.is_active(now)
    })
  }
}

pub struct TimeAllowanceRule {
  pub enabler: RuleEnabler,
  pub allowance: Duration,
}

impl TimeAllowanceRule {
  pub fn construct(
    enabler: RuleEnabler,
    allowance: Duration,
  ) -> Self {
    Self {
      enabler,
      allowance,
    }
  }

  pub fn is_enabled(&self, now: Instant) -> bool {
    self.enabler.is_rule_enabled(now)
  }

  pub fn is_active(&self, now: Instant, used_allowance: Duration) -> bool {
    self.is_enabled(now)
    &&
    used_allowance.is_longer_than_or_equal_to(self.allowance)
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeAllowanceRules {
  rules: HashMap<UuidV4, TimeAllowanceRules>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesStats {
  pub rules_number: usize,
  pub maximum_rules_number: usize,
}

impl RulesStats {
  pub fn new(maximum_rules_number: usize) -> Self {
    Self {
      rules_number: 0,
      maximum_rules_number,
    }
  }

  pub fn update_after_always_rule_created(&mut self) {}
  pub fn update_after_always_rule_deleted(&mut self) {}

  pub fn add_always_rule(&mut self) -> Result<(), ()> {
    todo!()
  }

  pub fn create_add_always_rule_updater(&self) -> Option<AddAlwaysRuleUpdater> {
    if self.rules_number < self.maximum_rules_number {
      Some(AddAlwaysRuleUpdater { rules_number: self.rules_number + 1 })
    } else {
      None
    }
  }
}

pub struct AddAlwaysRuleUpdater {
  rules_number: usize,
}
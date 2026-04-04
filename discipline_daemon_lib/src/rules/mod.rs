use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{Countdown, CountdownState, Duration, Instant, Time, TimeRange, UuidV4, TextualError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnablerType {
  Countdown = 0,
  CountdownAfterPlea = 1,
}

impl RuleEnablerType {
  pub fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      0 => {
        Ok(Self::Countdown)
      }
      1 => {
        Ok(Self::CountdownAfterPlea)
      }
      _ => {
        Err(TextualError::new("action"))
      }
    }
  }

  pub fn to_number(self) -> u8 {
    self as u8
  }
}

pub struct CountdownEnabler {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}

impl CountdownEnabler {
  pub fn is_rule_enabled(&self, time: Instant) -> bool {
    matches!(self.countdown, Some(countdown) if countdown.is_running(time))
  }

  pub fn enable_rule(&mut self, time: Instant) {
    self.countdown = Some(Countdown::construct(time, self.duration));
  }
}

pub struct CountdownAfterPleaEnabler {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownAfterPleaEnablerState {
  Active,
  Deactivating,
  Deactivated,
}

impl CountdownAfterPleaEnablerState {
  pub fn is_active(&self) -> bool {
    matches!(self, Self::Active)
  }
  
  pub fn is_deactivaing(&self) -> bool {
    matches!(self, Self::Deactivating)
  }
  
  pub fn is_deactivated(&self) -> bool {
    matches!(self, Self::Deactivated)
  }
}

impl CountdownAfterPleaEnabler {
  pub fn create(duration: Duration) -> Self {
    Self {
      duration,
      countdown: None,
    }
  }

  pub fn construct(duration: Duration, countdown: Option<Countdown>) -> Self {
    Self { 
      duration,
      countdown,
    }
  }
  
  pub fn get_state(&self, now: Instant) -> CountdownAfterPleaEnablerState {
    let Some(countdown) = &self.countdown else {
      return CountdownAfterPleaEnablerState::Active;
    };

    match countdown.get_state(now) {
      CountdownState::Pending => {
        CountdownAfterPleaEnablerState::Active
      }
      CountdownState::Running => {
        CountdownAfterPleaEnablerState::Deactivating
      }
      CountdownState::Finished => {
        CountdownAfterPleaEnablerState::Deactivated
      }
    }
  }

  pub fn is_rule_enabled(&self, time: Instant) -> bool {
    matches!(
      self.get_state(time), 
      CountdownAfterPleaEnablerState::Active 
      | 
      CountdownAfterPleaEnablerState::Deactivating,
    )
  }

  pub fn enable_rule(&mut self) {
    self.countdown = None;
  }

  pub fn disable_rule(&mut self, now: Instant) {
    self.countdown = Some(Countdown::construct(now, self.duration))
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnabler {
  Countdown(CountdownEnabler),
  CountdownAfterPlea(CountdownAfterPleaEnabler),
}

impl RuleEnabler {
  pub fn is_rule_enabled(&self, time: Instant) -> bool {
    match self {
      Self::Countdown(enabler) => {
        enabler.is_rule_enabled(time)
      }
      Self::CountdownAfterPlea(enabler) => {
        enabler.is_rule_enabled(time)
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
  pub fn is_enabled(&self, now: Instant) -> bool {
    self.enabler.is_rule_enabled(now)
  }

  pub fn is_active(&self, now: Instant) -> bool {
    self.enabler.is_rule_enabled(now)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlwaysRules {
  rules: HashMap<UuidV4, AlwaysRule>,
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

#[derive(Debug, Clone, Default)]
pub struct TimeAllowanceRules {
  rules: HashMap<UuidV4, TimeAllowanceRules>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesStats {
  rules_number: usize,
  maximum_rules_number: usize,
}

impl RulesStats {
  pub fn new(maximum_rules_number: usize) -> Self {
    Self {
      rules_number: 0,
      maximum_rules_number,
    }
  }
}
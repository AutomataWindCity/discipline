use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{Daemon, UuidV4, TimeConditional, AlwaysConditional, Time, Weekday, MonotonicInstant, CountdownConditional, CountdownAfterPleaConditional};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CachedRuleActivator {
  Time(TimeConditional),
  Always(AlwaysConditional),
}

impl CachedRuleActivator {
  pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
    match self {
      CachedRuleActivator::Time(inner) => {
        inner.evaulate(time, weekday)
      }
      CachedRuleActivator::Always(inner) => {
        inner.evaulate()
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CachedRuleEnabler {
  Countdown(CountdownConditional),
  CountdownAfterPlea(CountdownAfterPleaConditional),
}

impl CachedRuleEnabler {
  pub fn evaluate(&self, now: MonotonicInstant) -> bool {
    match self {
      CachedRuleEnabler::Countdown(inner) => {
        inner.is_activated(now)
      }
      CachedRuleEnabler::CountdownAfterPlea(inner) => {
        inner.is_activated_or_deactivating(now)
      }
    }
  }

  pub fn activate(&mut self, now: MonotonicInstant) {
    match self {
      CachedRuleEnabler::Countdown(inner) => {
        inner.activate(now);
      }
      CachedRuleEnabler::CountdownAfterPlea(inner) => {
        inner.activate()
      }
    }
  }

  pub fn deactivate(&mut self, now: MonotonicInstant) {
    match self {
      CachedRuleEnabler::Countdown(_inner) => {
        // noop
      }
      CachedRuleEnabler::CountdownAfterPlea(inner) => {
        inner.deactivate(now);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRule {
  activator: CachedRuleActivator,
  enabler: CachedRuleEnabler,  
}

impl CachedRule {
    pub fn new(
    activator: CachedRuleActivator,
    enabler: CachedRuleEnabler,
  ) -> Self {
    Self {
      activator,
      enabler,
    }
  }

  pub fn construct(
    activator: CachedRuleActivator,
    enabler: CachedRuleEnabler,
  ) -> Self {
    Self {
      activator,
      enabler,
    }
  }
  
  pub fn activator(&self) -> &CachedRuleActivator {
    &self.activator
  }

  pub fn enabler(&self) -> &CachedRuleEnabler {
    &self.enabler
  }
  pub fn enabler_mut(&mut self) -> &mut CachedRuleEnabler {
    &mut self.enabler
  }
  pub fn set_enabler(&mut self, new_value: CachedRuleEnabler) {
    self.enabler = new_value;
  }

  pub fn is_activated(&self, time: Time, weekday: Weekday) -> bool {
    self.activator.evaluate(time, weekday)
  }

  pub fn is_effective(
    &self, 
    now: MonotonicInstant,
    time: Time, 
    weekday: Weekday,
  ) -> bool {
    self.enabler.evaluate(now)
    &&
    self.activator.evaluate(time, weekday)
  }

  pub fn is_enabled(&self, now: MonotonicInstant) -> bool {
    self.enabler.evaluate(now)
  }

  pub fn activate(&mut self, now: MonotonicInstant) {
    self.enabler.activate(now);
  }

  pub fn deactivate(&mut self, now: MonotonicInstant) {
    self.enabler.deactivate(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleOwnerLocator {
  UserDeviceAccessRegulation {
    user_id: UuidV4,
  },
  UserInternetAccessRegulation {
    user_id: UuidV4,
  },
  UserAccountAccessRegulation {
    user_id: UuidV4,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRuleGroup {
  pub rules: HashMap<UuidV4, CachedRule>,
}

impl CachedRuleGroup {
  pub fn new() -> Self {
    Self {
      rules: HashMap::new(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossGroupInfo {
  pub rule_number: usize,
  pub maximum_rule_number: usize,
}

impl CrossGroupInfo {
  pub fn new(maximum_rule_number: usize) -> Self {
    Self {
      rule_number: 0,
      maximum_rule_number,
    }
  }
}

pub async fn add_rule(
  daemon: &Daemon,
  rule_id: UuidV4, 
  rule_activator: CachedRuleActivator,
  rule_enabler: CachedRuleEnabler,
  rule_owner_locator: RuleOwnerLocator,
) {
  match rule_owner_locator {
    RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
      let Some(user) = daemon
        .state
        .users
        .get_user(&user_id) else 
      {
        return;
      };

      let mut user = user.lock().await;

      let rule = CachedRule::new(
        rule_activator, 
        rule_enabler,
      );

      user
        .regulation
        .block_device_access
        .rules
        .rules
        .insert(rule_id, rule);
    }
    RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
      let Some(user) = daemon
        .state
        .users
        .get_user(&user_id) else 
      {
        return;
      };

      let user = user.lock().await;

      let rule = CachedRule::new(
        rule_activator, 
        rule_enabler,
      );

      // user
      //   .regulation
      //   .block_internet_access
      //   .rules
      //   .rules
      //   .insert(rule_id, rule);
    }
    RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
      let Some(user) = daemon
        .state
        .users
        .get_user(&user_id) else 
      {
        return;
      };

      let user = user.lock().await;

      let rule = CachedRule::new(
        rule_activator, 
        rule_enabler,
      );

      // user
      //   .regulation
      //   .block_account_access
      //   .rules
      //   .rules
      //   .insert(rule_id, rule);
    }
  }
}

pub async fn is_rule_enabled(
  daemon: &Daemon,
  now: MonotonicInstant,
  rule_id: &UuidV4,
  rule_owner_locator: &RuleOwnerLocator,
) -> bool {
  match rule_owner_locator {
    RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
      let Some(user) = daemon
        .state
        .users
        .get_user(user_id) else
      {
        return false
      };

      let user = user.lock().await;

      let Some(rule) = user
        .regulation
        .block_device_access
        .rules
        .rules
        .get(rule_id) else
      {
        return false
      };

      rule.is_enabled(now)
    }
    RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
            let Some(user) = daemon
        .state
        .users
        .get_user(user_id) else
      {
        return false
      };

      let user = user.lock().await;

      let Some(rule) = user
        .regulation
        .block_device_access
        .rules
        .rules
        .get(rule_id) else
      {
        return false
      };

      rule.is_enabled(now)
    }
    RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
            let Some(user) = daemon
        .state
        .users
        .get_user(user_id) else
      {
        return false
      };

      let user = user.lock().await;

      let Some(rule) = user
        .regulation
        .block_device_access
        .rules
        .rules
        .get(rule_id) else
      {
        return false
      };

      rule.is_enabled(now)
    }
  }
}

pub async fn delete_rule(
  daemon: &Daemon,
  rule_id: &UuidV4,
  rule_owner_locator: &RuleOwnerLocator,
) {
  match rule_owner_locator {
    RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
      let Some(user) = daemon
        .state
        .users
        .get_user(user_id) else 
      {
        return;
      };

      let mut user = user.lock().await;

      user
        .regulation
        .block_device_access
        .rules
        .rules
        .remove(rule_id);
    }
    RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {

    }
    RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {

    }
  }
}
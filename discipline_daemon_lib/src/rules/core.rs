use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, TimeConditional, AlwaysConditional, Time, Weekday, MonotonicInstant, CountdownConditional, CountdownAfterPleaConditional};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
  UserDeviceAccessRegulation { user_id: UuidV4 },
  UserAccountAccessRegulation { user_id: UuidV4 },
  UserInternetAccessRegulation { user_id: UuidV4 },
}

pub enum LocationRef<'a> {
  UserDeviceAccessRegulation { user_id: &'a UuidV4 },
  UserAccountAccessRegulation { user_id: &'a UuidV4 },
  UserInternetAccessRegulation { user_id: &'a UuidV4 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActivator {
  Time(TimeConditional),
  Always(AlwaysConditional),
}

impl RuleActivator {
  pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
    match self {
      RuleActivator::Time(inner) => {
        inner.evaulate(time, weekday)
      }
      RuleActivator::Always(inner) => {
        inner.evaulate()
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnabler {
  Countdown(CountdownConditional),
  CountdownAfterPlea(CountdownAfterPleaConditional),
}

impl RuleEnabler {
  pub fn evaluate(&self, now: MonotonicInstant) -> bool {
    match self {
      RuleEnabler::Countdown(inner) => {
        inner.is_activated(now)
      }
      RuleEnabler::CountdownAfterPlea(inner) => {
        inner.is_activated_or_deactivating(now)
      }
    }
  }

  pub fn activate(&mut self, now: MonotonicInstant) {
    match self {
      RuleEnabler::Countdown(inner) => {
        inner.activate(now);
      }
      RuleEnabler::CountdownAfterPlea(inner) => {
        inner.activate()
      }
    }
  }

  pub fn deactivate(&mut self, now: MonotonicInstant) {
    match self {
      RuleEnabler::Countdown(_inner) => {
        // noop
      }
      RuleEnabler::CountdownAfterPlea(inner) => {
        inner.deactivate(now);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
  activator: RuleActivator,
  pub(super) enabler: RuleEnabler,  
}

impl Rule {
    pub fn new(
    activator: RuleActivator,
    enabler: RuleEnabler,
  ) -> Self {
    Self {
      activator,
      enabler,
    }
  }

  pub fn construct(
    activator: RuleActivator,
    enabler: RuleEnabler,
  ) -> Self {
    Self {
      activator,
      enabler,
    }
  }
  
  pub fn activator(&self) -> &RuleActivator {
    &self.activator
  }

  pub fn enabler(&self) -> &RuleEnabler {
    &self.enabler
  }

  pub fn enabler_mut(&mut self) -> &mut RuleEnabler {
    &mut self.enabler
  }

  pub fn set_enabler(&mut self, new_value: RuleEnabler) {
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
pub struct RuleGroup {
  pub rules: HashMap<UuidV4, Rule>,
}

impl RuleGroup {
  pub fn new() -> Self {
    Self {
      rules: HashMap::new(),
    }
  }

  pub fn construct(rules: HashMap<UuidV4, Rule>) -> Self {
    Self { 
      rules,
    }
  }

  pub fn are_some_rules_enabled(&self, now: MonotonicInstant) -> bool {
    self.rules.values().any(|it| it.is_enabled(now))
  }

  pub fn add_rule(&mut self, rule_id: UuidV4, rule: Rule) {
    self.rules.insert(rule_id, rule);
  }

  pub fn get_rule_mut(&mut self, rule_id: &UuidV4) -> Option<&mut Rule> {
    self.rules.get_mut(rule_id)
  }

  pub fn delete_rule(&mut self, rule_id: &UuidV4) {
    self.rules.remove(rule_id);
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

  pub fn reached_maximum_rule_number(&self) -> bool {
    self.rule_number >= self.maximum_rule_number
  }

  pub fn increment_rule_number(&mut self) {
    self.rule_number = self.rule_number.saturating_add(1);
  }
  pub fn decrement_rule_number(&mut self) {
    self.rule_number = self.rule_number.saturating_sub(1);
  }
}

// pub async fn add_rule(
//   daemon: &Daemon,
//   rule_id: UuidV4, 
//   rule_activator: RuleActivator,
//   rule_enabler: RuleEnabler,
//   rule_owner_locator: RuleOwnerLocator,
// ) {
//   match rule_owner_locator {
//     RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
//       let Some(user) = daemon
//         .state
//         .users
//         .get_user(&user_id) else 
//       {
//         return;
//       };

//       let mut user = user.lock().await;

//       let rule = Rule::new(
//         rule_activator, 
//         rule_enabler,
//       );

//       user
//         .regulation
//         .block_device_access
//         .rules
//         .rules
//         .insert(rule_id, rule);
//     }
//     RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
//       let Some(user) = daemon
//         .state
//         .users
//         .get_user(&user_id) else 
//       {
//         return;
//       };

//       let user = user.lock().await;

//       let rule = Rule::new(
//         rule_activator, 
//         rule_enabler,
//       );

//       // user
//       //   .regulation
//       //   .block_internet_access
//       //   .rules
//       //   .rules
//       //   .insert(rule_id, rule);
//     }
//     RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
//       let Some(user) = daemon
//         .state
//         .users
//         .get_user(&user_id) else 
//       {
//         return;
//       };

//       let user = user.lock().await;

//       let rule = Rule::new(
//         rule_activator, 
//         rule_enabler,
//       );

//       // user
//       //   .regulation
//       //   .block_account_access
//       //   .rules
//       //   .rules
//       //   .insert(rule_id, rule);
//     }
//   }
// }

// pub async fn is_rule_enabled(
//   daemon: &Daemon,
//   now: MonotonicInstant,
//   rule_id: &UuidV4,
//   rule_owner_locator: &RuleOwnerLocator,
// ) -> bool {
//   match rule_owner_locator {
//     RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
//       let Some(user) = daemon
//         .state
//         .users
//         .get_user(user_id) else
//       {
//         return false
//       };

//       let user = user.lock().await;

//       let Some(rule) = user
//         .regulation
//         .block_device_access
//         .rules
//         .rules
//         .get(rule_id) else
//       {
//         return false
//       };

//       rule.is_enabled(now)
//     }
//     RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
//             let Some(user) = daemon
//         .state
//         .users
//         .get_user(user_id) else
//       {
//         return false
//       };

//       let user = user.lock().await;

//       let Some(rule) = user
//         .regulation
//         .block_device_access
//         .rules
//         .rules
//         .get(rule_id) else
//       {
//         return false
//       };

//       rule.is_enabled(now)
//     }
//     RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
//             let Some(user) = daemon
//         .state
//         .users
//         .get_user(user_id) else
//       {
//         return false
//       };

//       let user = user.lock().await;

//       let Some(rule) = user
//         .regulation
//         .block_device_access
//         .rules
//         .rules
//         .get(rule_id) else
//       {
//         return false
//       };

//       rule.is_enabled(now)
//     }
//   }
// }

// pub async fn delete_rule(
//   daemon: &Daemon,
//   rule_id: &UuidV4,
//   rule_owner_locator: &RuleOwnerLocator,
// ) {
//   match rule_owner_locator {
//     RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
//       let Some(user) = daemon
//         .state
//         .users
//         .get_user(user_id) else 
//       {
//         return;
//       };

//       let mut user = user.lock().await;

//       user
//         .regulation
//         .block_device_access
//         .rules
//         .rules
//         .remove(rule_id);
//     }
//     RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {

//     }
//     RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {

//     }
//   }
// }

// pub async fn get_rule(
//   daemon: &Daemon, 
//   rule_id: &UuidV4, 
//   rule_owner_locator: &RuleOwnerLocator,
// ) -> Option<Rule> {
//   todo!()
// }
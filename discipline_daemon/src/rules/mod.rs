use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, MonotonicInstant, Time, TimeConditional, UuidV4, Weekday, always_conditional, countdown_after_plea_conditional, countdown_conditional, time_conditional};

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
  enabler: RuleEnabler,
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
pub struct RuleGroup {
  pub rules: HashMap<UuidV4, Rule>,
  pub maximum_rule_number: usize,
}

impl RuleGroup {
  pub fn new(maximum_rule_number: usize) -> Self {
    Self {
      rules: HashMap::new(),
      maximum_rule_number,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActivatorCreator {
  Time(time_conditional::Creator),
  Always(always_conditional::Creator)
}

impl RuleActivatorCreator {
  pub fn create(self) -> RuleActivator {
    match self {
      Self::Time(creator) => {
        RuleActivator::Time(creator.create())
      }
      Self::Always(creator) => {
        RuleActivator::Always(creator.create())
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnablerCreator {
  Countdown(countdown_conditional::Creator),
  CountdownAfterPlea(countdown_after_plea_conditional::Creator)
}

impl RuleEnablerCreator {
  pub fn create(self) -> RuleEnabler {
    match self {
      Self::Countdown(creator) => {
        RuleEnabler::Countdown(creator.create())
      }
      Self::CountdownAfterPlea(creator) => {
        RuleEnabler::CountdownAfterPlea(creator.create())
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreator {
  pub id: Option<UuidV4>,
  pub activator: RuleActivatorCreator,
  pub enabler: RuleEnablerCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleError {
  BadUuidV4TryAgain,
  ReachedMaximumRulesAllowedDeleteSomeAndTryAgain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleSuccess {
  NoSuchRule,
  Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleError {
  RuleIsProtected,
}

impl RuleGroup {
  pub fn add_rule_given_rule_creator(
    &mut self, 
    rule_creator: RuleCreator,
  ) -> Result<(UuidV4, Rule), AddRuleError> {
    if self.rules.len() >= self.maximum_rule_number {
      return Err(AddRuleError::ReachedMaximumRulesAllowedDeleteSomeAndTryAgain);
    }

    let id = rule_creator.id.unwrap_or_else(UuidV4::generate);
    if self.rules.contains_key(&id) {
      return Err(AddRuleError::BadUuidV4TryAgain);
    }

    let rule = Rule::new(
      rule_creator.activator.create(),
      rule_creator.enabler.create(),
    );

    self.rules.insert(id.clone(), rule.clone());
    Ok((id, rule))
  }

  pub fn delete_rule_given_rule_id(
    &mut self, 
    now: MonotonicInstant,
    rule_id: &UuidV4,
  ) -> Result<DeleteRuleSuccess, DeleteRuleError> {
    let Some(rule) = self.rules.get_mut(rule_id) else {
      return Ok(DeleteRuleSuccess::NoSuchRule);
    };

    if rule.is_enabled(now) {
      return Err(DeleteRuleError::RuleIsProtected);
    }

    self.rules.remove(rule_id);
    Ok(DeleteRuleSuccess::Success)
  }

  pub fn force_delete_rule_if_exists(&mut self, rule_id: &UuidV4) {
    self.rules.remove(rule_id);
  }
}

// pub mod procedures {
//   use crate::{rules::rules_x::{RuleEnablerX, RuleX}, x::{InstantX, RuleEnabler, countdown_after_plea_conditional_x, countdown_conditional, countdown_conditional_x}};

//   pub enum EnablerProcedure {
//     Countdown(countdown_conditional_x::procedures::Procedure),
//     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Procedure),
//   }

//   pub enum EnablerProcedureReturn {
//     VariantMismatch,
//     Countdown(countdown_conditional_x::procedures::Return),
//     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Return),
//   }
  
//   impl EnablerProcedure {
//     pub fn execute(
//       self,
//       instant: InstantX,
//       conditional: &mut RuleEnablerX,
//     ) -> EnablerProcedureReturn {
//       match (self, conditional) {
//         (
//           EnablerProcedure::Countdown(operation), 
//           RuleEnablerX::Countdown(conditional)
//         ) => {
//           EnablerProcedureReturn::Countdown(
//             operation.execute(instant, conditional)
//           )
//         }
//         (
//           EnablerProcedure::CountdownAfterPlea(operation), 
//           RuleEnablerX::CountdownAfterPlea(conditional)
//         ) => {
//           EnablerProcedureReturn::CountdownAfterPlea(
//             operation.execute(instant, conditional)
//           )
//         }
//         _ => {
//           EnablerProcedureReturn::VariantMismatch
//         }
//       }
//     }
//   }

//   pub struct Activate {

//   }

//   pub trait DatabaseProcedures {
//     fn add_rule(&self, rule: &RuleX) -> Result<(), ()>;
//   }
  
//   pub struct CreateRule {
//     // rule_creator
//   }
// }


// // pub mod rules_x;

// // use std::collections::HashMap;
// // use serde::{Deserialize, Serialize};
// // use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, DateTime, Time, TimeConditional, UuidV4, Weekday};

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum RuleActivator {
// //   Time(TimeConditional),
// //   Alwaus(AlwaysConditional),
// // }

// // impl RuleActivator {
// //   pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
// //     match self {
// //       RuleActivator::Time(inner) => inner.evaulate(time, weekday),
// //       RuleActivator::Alwaus(inner) => inner.evaulate(),
// //     }
// //   }
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum RuleEnabler {
// //   Countdown(CountdownConditional),
// //   CountdownAfterPlea(CountdownAfterPleaConditional),
// // }

// // impl RuleEnabler {
// //   pub fn evaluate(&self) -> bool {
// //     match self {
// //       RuleEnabler::Countdown(inner) => inner.is_activated(),
// //       RuleEnabler::CountdownAfterPlea(inner) => inner.is_activated_or_deactivating(),
// //     }
// //   }

// //   pub fn synchronize(&mut self, now: DateTime) {
// //     match self {
// //       RuleEnabler::Countdown(inner) => inner.synchronize(now),
// //       RuleEnabler::CountdownAfterPlea(inner) => inner.synchronize(now),
// //     }
// //   }

// //   pub fn activate(&mut self, now: DateTime) {
// //     match self {
// //       RuleEnabler::Countdown(inner) => inner.activate(now),
// //       RuleEnabler::CountdownAfterPlea(inner) => inner.activate(),
// //     }
// //   }

// //   pub fn deactivate(&mut self, now: DateTime) {
// //     match self {
// //       RuleEnabler::Countdown(_inner) => {
// //         // noop
// //       }
// //       RuleEnabler::CountdownAfterPlea(inner) => {
// //         inner.deactivate(now);
// //       }
// //     }
// //   }
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub struct Rule {
// //   activator: RuleActivator,
// //   enabler: RuleEnabler,
// //   is_activated: bool,
// // }

// // impl Rule {
// //   pub fn new(
// //     activator: RuleActivator,
// //     enabler: RuleEnabler,
// //   ) -> Self {
// //     Self {
// //       activator,
// //       enabler,
// //       is_activated: false,
// //     }
// //   }

// //   pub fn construct(
// //     is_activated: bool,
// //     activator: RuleActivator,
// //     enabler: RuleEnabler,
// //   ) -> Self {
// //     Self {
// //       is_activated,
// //       activator,
// //       enabler,
// //     }
// //   }
  
// //   pub fn activator(&self) -> &RuleActivator {
// //     &self.activator
// //   }

// //   pub fn enabler(&self) -> &RuleEnabler {
// //     &self.enabler
// //   }

// //   pub fn is_activated(&self) -> bool {
// //     self.is_activated
// //   }

// //   pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
// //     self.is_activated
// //     && 
// //     self.activator.evaluate(time, weekday)
// //   }

// //   pub fn is_protected(&self) -> bool {
// //     self.enabler.evaluate()
// //   }
// // }

// // pub struct RuleGroup {
// //   rules: HashMap<UuidV4, Rule>,
// //   maximum_rule_number: usize,
// // }

// // impl RuleGroup {
// //   pub fn new(maximum_rule_number: usize) -> Self {
// //     Self {
// //       rules: HashMap::new(),
// //       maximum_rule_number,
// //     }
// //   }
// // }

// // pub mod operations {
// //   use crate::x::{RuleEnabler, UuidV4, countdown_after_plea_conditional, countdown_conditional};

// //   // DaemonUsersRegulationBlockInternetAddRule
// //   // DaemonUsersRegulationBlockInternetDeleteRule
// //   // DaemonUsersRegulationBlockInternetActivateRule
// //   // DaemonUsersRegulationBlockInternetDeactivateRule

// //   // EnablerTypeMismatch
// //   // AlreadyActivated

// //   pub enum RuleEnablerActivate {
// //     Countdown(countdown_conditional::operations::Activate),
// //     CountdownAfterPlea(countdown_after_plea_conditional::operations::Activate),
// //   }

// //   pub enum RuleEnablerActivateReturn {
// //     TypeMismatch,
// //     Countdown(countdown_conditional::operations::ActivateReturn),
// //     CountdownAfterPlea(countdown_after_plea_conditional::operations::ActivateReturn),
// //   }
  
// //   impl RuleEnablerActivate {
// //     pub fn execute(
// //       self,
// //       conditional: &mut RuleEnabler,
// //     ) -> RuleEnablerActivateReturn {
// //       match (self, conditional) {
// //         (
// //           RuleEnablerActivate::Countdown(operation), 
// //           RuleEnabler::Countdown(conditional)
// //         ) => {
// //           RuleEnablerActivateReturn::Countdown(
// //             operation.execute(conditional)
// //           )
// //         }
// //         (
// //           RuleEnablerActivate::CountdownAfterPlea(operation), 
// //           RuleEnabler::CountdownAfterPlea(conditional)
// //         ) => {
// //           RuleEnablerActivateReturn::CountdownAfterPlea(
// //             operation.execute(conditional)
// //           )
// //         }
// //         _ => {
// //           RuleEnablerActivateReturn::TypeMismatch
// //         }
// //       }
// //     }
// //   }

// //   pub struct Activate {

// //   }
// // }
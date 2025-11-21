use serde::{Serialize, Deserialize};
use crate::x::{Daemon, UuidV4};
use crate::x::rules::{database, cache, RuleCreator, RuleOwnerLocator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  rule_creator: RuleCreator,
  rule_owner_locator: RuleOwnerLocator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleReturn {
  NoSuchOwner,
  DuplicateUuid,
  TooManyRules,
  InternalError,
  Success,
}

impl AddRule {
  pub async fn execute(self, daemon: &Daemon) -> AddRuleReturn {
    let mut info = daemon.state.rules.lock().await;
    if info.rule_number >= info.maximum_rule_number {
      return AddRuleReturn::TooManyRules;
    }
    
    let rule_id = self
      .rule_creator
      .id
      .unwrap_or_else(UuidV4::generate);

    let rule_activator = self
      .rule_creator
      .activator
      .create();

    let rule_enabler = self
      .rule_creator
      .enabler
      .create();

    if let Err(error) = database::add_rule(
      &daemon.database,
      &rule_id, 
      &rule_activator,
      &rule_enabler,
      &self.rule_owner_locator,
    ).await {
      return match error {
        database::AddRuleError::DuplicateId => {
          AddRuleReturn::DuplicateUuid
        }
        database::AddRuleError::NoSuchOwner => {
          AddRuleReturn::NoSuchOwner
        }
        database::AddRuleError::Other => {
          AddRuleReturn::InternalError
        }
      };
    }
    
    cache::add_rule(
      daemon, 
      rule_id, 
      rule_activator, 
      rule_enabler, 
      self.rule_owner_locator,
    );

    info.rule_number += 1;

    AddRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRule {
  rule_id: UuidV4,
  rule_owner_locator: RuleOwnerLocator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  RuleStillEnabled,
  NoSuchRule,
  NoSuchOwner,
  InternalError,
  Success,
}

impl DeleteRule {
  pub async fn execute(self, daemon: &Daemon) -> DeleteRuleReturn {
    let instant = daemon.state.clock.now();

    if cache::is_rule_enabled(
      daemon, 
      instant, 
      &self.rule_id, 
      &self.rule_owner_locator,
    ).await {
      return DeleteRuleReturn::RuleStillEnabled;
    }

    if let Err(error) = database::remove_rule(
      &daemon.database,
      &self.rule_id,
      &self.rule_owner_locator,
    ).await {
      return match error {
        database::RemoveRuleError::NoSuchRule => {
          DeleteRuleReturn::NoSuchRule
        }
        database::RemoveRuleError::Other => {
          DeleteRuleReturn::InternalError
        }
      };
    }

    cache::delete_rule(
      daemon, 
      &self.rule_id, 
      &self.rule_owner_locator,
    );
  
    DeleteRuleReturn::Success
  }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateRule {
//   rule_id: UuidV4,
//   updates: Vec<RuleUpdate>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum UpdateRuleFailure {
//   NoSuchRule,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateRuleSuccess {
//   successes: Vec<RuleUpdateSuccess>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct RevertUpdateRule {
//   rule_id: UuidV4,
//   rule_original_state: Rule,
// }

// impl UpdateRule {
//   pub fn execute(
//     self,
//     transaction: &mut impl Transaction,
//     writer: &impl TransactionWriter,
//     rule_group: &mut RuleGroup,
//     now: MonotonicInstant,
//   ) -> Result<(UpdateRuleSuccess, RevertUpdateRule), UpdateRuleFailure> {
//     let rule_id = self.rule_id;

//     let Some(rule) = rule_group.rules.get_mut(&rule_id) else {
//       return Err(UpdateRuleFailure::NoSuchRule);
//     };

//     let rule_original_state = rule.clone();

//     let successes = self
//       .updates
//       .into_iter()
//       .map(|it| it.execute(rule, now))
//       .collect();

//     writer.update_rule(
//       transaction, 
//       &rule_id, 
//       &rule_original_state, 
//       &rule,
//     );

//     Ok((
//       UpdateRuleSuccess { successes },
//       RevertUpdateRule { rule_id, rule_original_state }
//     ))
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ActivateRule;

// impl ActivateRule {
//   pub fn execute(
//     self,
//     rule: &mut Rule,
//     now: MonotonicInstant,
//   ) {
//     rule.activate(now);
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DeactivateRule;

// impl DeactivateRule {
//   pub fn execute(
//     self,
//     rule: &mut Rule,
//     now: MonotonicInstant,
//   ) {
//     rule.deactivate(now);
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum RuleUpdate {
//   Activate(ActivateRule),
//   Deactivate(DeactivateRule),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum RuleUpdateSuccess {
//   Activate,
//   Deactivate,
// }

// impl RuleUpdate {
//   pub fn execute(
//     self, 
//     rule: &mut Rule,
//     now: MonotonicInstant,
//   ) -> RuleUpdateSuccess {
//     match self {
//       Self::Activate(proceduer) => {
//         proceduer.execute(rule, now);
//         RuleUpdateSuccess::Activate
//       }
//       Self::Deactivate(procedure) => {
//         procedure.execute(rule, now);
//         RuleUpdateSuccess::Deactivate
//       }
//     }
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum Procedure {
//   AddRule(AddRule),
//   EnsureRuleDeleted(EnsureRuleDeleted),
//   UpdateRule(UpdateRule),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProcedureSuccess {
//   AddRule,
//   EnsureRuleDeleted,
//   UpdateRule(UpdateRuleSuccess),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProcedureFailure {
//   AddRule(AddRuleFailure),
//   EnsureRuleDeleted(EnsureRuleDeletedFailure),
//   UpdateRule(UpdateRuleFailure),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProcedureRevert {
//   AddRule(RevertAddRule),
//   EnsureRuleDeleted(Option<RevertDeleteRule>),
//   UpdateRule(RevertUpdateRule),
// }

// impl Procedure {
//   pub fn execute(
//     self,
//     transaction: &mut impl Transaction,
//     writer: &impl TransactionWriter,
//     rule_group: &mut RuleGroup,
//     now: MonotonicInstant,
//   ) -> Result<(ProcedureSuccess, ProcedureRevert), ProcedureFailure> {
//     match self {
//       Procedure::AddRule(inner) => {
//         inner.execute(transaction, writer, rule_group) 
//           .map(|revert| (
//             ProcedureSuccess::AddRule,
//             ProcedureRevert::AddRule(revert),
//           ))
//           .map_err(ProcedureFailure::AddRule)
//       }
//       Procedure::EnsureRuleDeleted(inner) => {
//         inner.execute(transaction, writer, rule_group, now)
//           .map(|revert| (
//             ProcedureSuccess::EnsureRuleDeleted,
//             ProcedureRevert::EnsureRuleDeleted(revert),
//           ))
//           .map_err(ProcedureFailure::EnsureRuleDeleted)
//       }
//       Procedure::UpdateRule(inner) => {
//         inner.execute(transaction, writer, rule_group, now)
//           .map(|(success, revert)| (
//             ProcedureSuccess::UpdateRule(success),
//             ProcedureRevert::UpdateRule(revert),
//           ))
//           .map_err(ProcedureFailure::UpdateRule)
//       }
//     }
//   }
// }



// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum AddRuleError {
//   BadUuidV4TryAgain,
//   ReachedMaximumRulesAllowedDeleteSomeAndTryAgain,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeleteRuleSuccess {
//   NoSuchRule,
//   Success,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeleteRuleError {
//   RuleIsProtected,
// }

// impl CachedRuleGroup {
//   pub fn add_rule_given_rule_creator(
//     &mut self, 
//     rule_creator: RuleCreator,
//   ) -> Result<(UuidV4, Rule), AddRuleError> {
//     if self.rules.len() >= self.maximum_rule_number {
//       return Err(AddRuleError::ReachedMaximumRulesAllowedDeleteSomeAndTryAgain);
//     }

//     let id = rule_creator.id.unwrap_or_else(UuidV4::generate);
//     if self.rules.contains_key(&id) {
//       return Err(AddRuleError::BadUuidV4TryAgain);
//     }

//     let rule = Rule::new(
//       rule_creator.activator.create(),
//       rule_creator.enabler.create(),
//     );

//     self.rules.insert(id.clone(), rule.clone());
//     Ok((id, rule))
//   }

//   pub fn delete_rule_given_rule_id(
//     &mut self, 
//     now: MonotonicInstant,
//     rule_id: &UuidV4,
//   ) -> Result<DeleteRuleSuccess, DeleteRuleError> {
//     let Some(rule) = self.rules.get_mut(rule_id) else {
//       return Ok(DeleteRuleSuccess::NoSuchRule);
//     };

//     if rule.is_enabled(now) {
//       return Err(DeleteRuleError::RuleIsProtected);
//     }

//     self.rules.remove(rule_id);
//     Ok(DeleteRuleSuccess::Success)
//   }

//   pub fn force_delete_rule_if_exists(&mut self, rule_id: &UuidV4) {
//     self.rules.remove(rule_id);
//   }
// }

// // pub mod procedures {
// //   use crate::{rules::rules_x::{RuleEnablerX, RuleX}, x::{InstantX, RuleEnabler, countdown_after_plea_conditional_x, countdown_conditional, countdown_conditional_x}};

// //   pub enum EnablerProcedure {
// //     Countdown(countdown_conditional_x::procedures::Procedure),
// //     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Procedure),
// //   }

// //   pub enum EnablerProcedureReturn {
// //     VariantMismatch,
// //     Countdown(countdown_conditional_x::procedures::Return),
// //     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Return),
// //   }
  
// //   impl EnablerProcedure {
// //     pub fn execute(
// //       self,
// //       instant: InstantX,
// //       conditional: &mut RuleEnablerX,
// //     ) -> EnablerProcedureReturn {
// //       match (self, conditional) {
// //         (
// //           EnablerProcedure::Countdown(operation), 
// //           RuleEnablerX::Countdown(conditional)
// //         ) => {
// //           EnablerProcedureReturn::Countdown(
// //             operation.execute(instant, conditional)
// //           )
// //         }
// //         (
// //           EnablerProcedure::CountdownAfterPlea(operation), 
// //           RuleEnablerX::CountdownAfterPlea(conditional)
// //         ) => {
// //           EnablerProcedureReturn::CountdownAfterPlea(
// //             operation.execute(instant, conditional)
// //           )
// //         }
// //         _ => {
// //           EnablerProcedureReturn::VariantMismatch
// //         }
// //       }
// //     }
// //   }

// //   pub struct Activate {

// //   }

// //   pub trait DatabaseProcedures {
// //     fn add_rule(&self, rule: &RuleX) -> Result<(), ()>;
// //   }
  
// //   pub struct CreateRule {
// //     // rule_creator
// //   }
// // }


// // // pub mod rules_x;

// // // use std::collections::HashMap;
// // // use serde::{Deserialize, Serialize};
// // // use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, DateTime, Time, TimeConditional, UuidV4, Weekday};

// // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // pub enum RuleActivator {
// // //   Time(TimeConditional),
// // //   Alwaus(AlwaysConditional),
// // // }

// // // impl RuleActivator {
// // //   pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
// // //     match self {
// // //       RuleActivator::Time(inner) => inner.evaulate(time, weekday),
// // //       RuleActivator::Alwaus(inner) => inner.evaulate(),
// // //     }
// // //   }
// // // }

// // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // pub enum RuleEnabler {
// // //   Countdown(CountdownConditional),
// // //   CountdownAfterPlea(CountdownAfterPleaConditional),
// // // }

// // // impl RuleEnabler {
// // //   pub fn evaluate(&self) -> bool {
// // //     match self {
// // //       RuleEnabler::Countdown(inner) => inner.is_activated(),
// // //       RuleEnabler::CountdownAfterPlea(inner) => inner.is_activated_or_deactivating(),
// // //     }
// // //   }

// // //   pub fn synchronize(&mut self, now: DateTime) {
// // //     match self {
// // //       RuleEnabler::Countdown(inner) => inner.synchronize(now),
// // //       RuleEnabler::CountdownAfterPlea(inner) => inner.synchronize(now),
// // //     }
// // //   }

// // //   pub fn activate(&mut self, now: DateTime) {
// // //     match self {
// // //       RuleEnabler::Countdown(inner) => inner.activate(now),
// // //       RuleEnabler::CountdownAfterPlea(inner) => inner.activate(),
// // //     }
// // //   }

// // //   pub fn deactivate(&mut self, now: DateTime) {
// // //     match self {
// // //       RuleEnabler::Countdown(_inner) => {
// // //         // noop
// // //       }
// // //       RuleEnabler::CountdownAfterPlea(inner) => {
// // //         inner.deactivate(now);
// // //       }
// // //     }
// // //   }
// // // }

// // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // pub struct Rule {
// // //   activator: RuleActivator,
// // //   enabler: RuleEnabler,
// // //   is_activated: bool,
// // // }

// // // impl Rule {
// // //   pub fn new(
// // //     activator: RuleActivator,
// // //     enabler: RuleEnabler,
// // //   ) -> Self {
// // //     Self {
// // //       activator,
// // //       enabler,
// // //       is_activated: false,
// // //     }
// // //   }

// // //   pub fn construct(
// // //     is_activated: bool,
// // //     activator: RuleActivator,
// // //     enabler: RuleEnabler,
// // //   ) -> Self {
// // //     Self {
// // //       is_activated,
// // //       activator,
// // //       enabler,
// // //     }
// // //   }
  
// // //   pub fn activator(&self) -> &RuleActivator {
// // //     &self.activator
// // //   }

// // //   pub fn enabler(&self) -> &RuleEnabler {
// // //     &self.enabler
// // //   }

// // //   pub fn is_activated(&self) -> bool {
// // //     self.is_activated
// // //   }

// // //   pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
// // //     self.is_activated
// // //     && 
// // //     self.activator.evaluate(time, weekday)
// // //   }

// // //   pub fn is_protected(&self) -> bool {
// // //     self.enabler.evaluate()
// // //   }
// // // }

// // // pub struct RuleGroup {
// // //   rules: HashMap<UuidV4, Rule>,
// // //   maximum_rule_number: usize,
// // // }

// // // impl RuleGroup {
// // //   pub fn new(maximum_rule_number: usize) -> Self {
// // //     Self {
// // //       rules: HashMap::new(),
// // //       maximum_rule_number,
// // //     }
// // //   }
// // // }

// // // pub mod operations {
// // //   use crate::x::{RuleEnabler, UuidV4, countdown_after_plea_conditional, countdown_conditional};

// // //   // DaemonUsersRegulationBlockInternetAddRule
// // //   // DaemonUsersRegulationBlockInternetDeleteRule
// // //   // DaemonUsersRegulationBlockInternetActivateRule
// // //   // DaemonUsersRegulationBlockInternetDeactivateRule

// // //   // EnablerTypeMismatch
// // //   // AlreadyActivated

// // //   pub enum RuleEnablerActivate {
// // //     Countdown(countdown_conditional::operations::Activate),
// // //     CountdownAfterPlea(countdown_after_plea_conditional::operations::Activate),
// // //   }

// // //   pub enum RuleEnablerActivateReturn {
// // //     TypeMismatch,
// // //     Countdown(countdown_conditional::operations::ActivateReturn),
// // //     CountdownAfterPlea(countdown_after_plea_conditional::operations::ActivateReturn),
// // //   }
  
// // //   impl RuleEnablerActivate {
// // //     pub fn execute(
// // //       self,
// // //       conditional: &mut RuleEnabler,
// // //     ) -> RuleEnablerActivateReturn {
// // //       match (self, conditional) {
// // //         (
// // //           RuleEnablerActivate::Countdown(operation), 
// // //           RuleEnabler::Countdown(conditional)
// // //         ) => {
// // //           RuleEnablerActivateReturn::Countdown(
// // //             operation.execute(conditional)
// // //           )
// // //         }
// // //         (
// // //           RuleEnablerActivate::CountdownAfterPlea(operation), 
// // //           RuleEnabler::CountdownAfterPlea(conditional)
// // //         ) => {
// // //           RuleEnablerActivateReturn::CountdownAfterPlea(
// // //             operation.execute(conditional)
// // //           )
// // //         }
// // //         _ => {
// // //           RuleEnablerActivateReturn::TypeMismatch
// // //         }
// // //       }
// // //     }
// // //   }

// // //   pub struct Activate {

// // //   }
// // // }
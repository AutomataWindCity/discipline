use serde::{Serialize, Deserialize};
use crate::x::{MonotonicInstant, UuidV4, Database};
use crate::x::rules::{Rule, RuleGroup, RuleCreator, RuleActivator, RuleEnabler, CrossGroupInfo, Location};

pub enum DbAddRuleError {
  DuplicateId,
  InternalError,
}

async fn db_add_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
  rule_enabler: &RuleEnabler,
  rule_activator: &RuleActivator,
) -> Result<(), DbAddRuleError> {
  todo!()
}

pub enum DbDeleteRuleError {
  NoSuchRule,
  InternalError,
}

async fn db_delete_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
) -> Result<(), DbDeleteRuleError> {
  todo!()
}

pub enum DbUpdateRuleEnablerError {
  NoSuchRule,
  InternalError,
}

async fn db_update_rule_enabler(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
  original_enabler: &RuleEnabler,
  updated_enabler: &RuleEnabler,
) -> Result<(), DbUpdateRuleEnablerError> {
  todo!()
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  creator: RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleReturn {
  TooManyRules,
  DuplicateId,
  InternalError,
  Success,
}

impl AddRule {
  pub async fn execute(
    self, 
    location: &Location,
    database: &Database,
    rule_group: &mut RuleGroup,
    cross_group_info: &mut CrossGroupInfo,
  ) -> AddRuleReturn {
    if cross_group_info.rule_number >= cross_group_info.maximum_rule_number {
      return AddRuleReturn::TooManyRules;
    }

    let id_was_created_by_client = self.creator.id.is_some();
    let rule_id = self.creator.id.unwrap_or_else(UuidV4::generate);
    let rule_enabler = self.creator.enabler.create();
    let rule_activator = self.creator.activator.create();

    if let Err(error) = db_add_rule(
      database,
      location,
      &rule_id, 
      &rule_enabler, 
      &rule_activator, 
    ).await {
      return match error {
        DbAddRuleError::DuplicateId => {
          if id_was_created_by_client {
            AddRuleReturn::DuplicateId
          } else {
            AddRuleReturn::InternalError
          }
        }
        DbAddRuleError::InternalError => {
          AddRuleReturn::InternalError
        }
      };
    }

    rule_group.rules.insert(rule_id, Rule::new(
      rule_activator, 
      rule_enabler,
    ));

    cross_group_info.rule_number += 1;

    AddRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRule {
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  NoSuchRule,
  RuleStillEnabled,
  InternalError,
  Success,
}

impl DeleteRule {
  pub async fn execute(
    self,
    now: MonotonicInstant,
    location: &Location,
    database: &Database,
    rule_group: &mut RuleGroup,
    cross_group_info: &mut CrossGroupInfo,
  ) -> DeleteRuleReturn {
    let Some(rule) = rule_group.rules.get(&self.rule_id) else {
      return DeleteRuleReturn::NoSuchRule;
    };

    if rule.is_enabled(now) {
      return DeleteRuleReturn::RuleStillEnabled;
    }

    if let Err(error) = db_delete_rule(database, location, &self.rule_id).await {
      return match error {
        DbDeleteRuleError::InternalError => DeleteRuleReturn::InternalError,
        DbDeleteRuleError::NoSuchRule => DeleteRuleReturn::InternalError,
      };
    }

    rule_group.rules.remove(&self.rule_id);
    cross_group_info.rule_number -= 1;

    DeleteRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateRule {
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivateRuleReturn {
  NoSuchRule,
  InternalError,
  Success,
}

impl ActivateRule {
  pub async fn execute(
    self,
    now: MonotonicInstant,
    location: &Location,
    database: &Database,
    rule_group: &mut RuleGroup,
  ) -> ActivateRuleReturn {
    let Some(rule) = rule_group.rules.get_mut(&self.rule_id) else {
      return ActivateRuleReturn::NoSuchRule;
    };

    let mut enabler = rule.enabler.clone();
    enabler.activate(now);

    if let Err(error) = db_update_rule_enabler(
      database, 
      location, 
      &self.rule_id, 
      &rule.enabler, 
      &enabler,
    ).await {
      return match error {
        DbUpdateRuleEnablerError::InternalError => ActivateRuleReturn::InternalError,
        DbUpdateRuleEnablerError::NoSuchRule => ActivateRuleReturn::InternalError,
      };
    }

    rule.enabler = enabler;
    ActivateRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivateRule {
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeactivateRuleReturn {
  NoSuchRule,
  InternalError,
  Success,
}

impl DeactivateRule {
  pub async fn execute(
    self, 
    now: MonotonicInstant,
    location: &Location,
    database: &Database,
    rule_group: &mut RuleGroup,
  ) -> DeactivateRuleReturn {
    let Some(rule) = rule_group.rules.get_mut(&self.rule_id) else {
      return DeactivateRuleReturn::NoSuchRule;
    };

    let mut enabler = rule.enabler.clone();
    enabler.deactivate(now);

    if let Err(error) = db_update_rule_enabler(
      database, 
      location, 
      &self.rule_id, 
      &rule.enabler, 
      &enabler,
    ).await {
      return match error {
        DbUpdateRuleEnablerError::InternalError => DeactivateRuleReturn::InternalError,
        DbUpdateRuleEnablerError::NoSuchRule => DeactivateRuleReturn::InternalError,
      };
    }

    rule.enabler = enabler;
    DeactivateRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyProcedure {
  AddRule(AddRule),
  DeleteRule(DeleteRule),
  ActivateRule(ActivateRule),
  DeactivateRule(DeactivateRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyProcedureReturn {
  AddRule(AddRuleReturn),
  DeleteRule(DeleteRuleReturn),
  ActivateRule(ActivateRuleReturn),
  DeactivateRule(DeactivateRuleReturn),
}

impl AnyProcedure {
  pub async fn execute(
    self,
    now: MonotonicInstant,
    location: &Location,
    database: &Database,
    rule_group: &mut RuleGroup,
    cross_group_info: &mut CrossGroupInfo,
  ) -> AnyProcedureReturn {
    match self {
      AnyProcedure::AddRule(inner) => {
        AnyProcedureReturn::AddRule(inner.execute(location, database, rule_group, cross_group_info).await)
      }
      AnyProcedure::DeleteRule(inner) => {
        AnyProcedureReturn::DeleteRule(inner.execute(now, location, database, rule_group, cross_group_info).await)
      }
      AnyProcedure::ActivateRule(inner) => {
        AnyProcedureReturn::ActivateRule(inner.execute(now, location, database, rule_group).await)
      }
      AnyProcedure::DeactivateRule(inner) => {
        AnyProcedureReturn::DeactivateRule(inner.execute(now, location, database, rule_group).await)
      }
    }
  }
}

// async fn get_rule(
//   daemon: &Daemon,
//   rule_id: &UuidV4,
//   rule_owner_locator: &RuleOwnerLocator,
// ) -> Result<super::CachedRule, ActivateRuleReturn> {
//   if let Some(rule) = cache::get_rule(daemon, rule_id, rule_owner_locator).await {
//     return Ok(rule);
//   };

//   let error = match database::get_cached_rule(
//     &daemon.database, 
//     rule_id, 
//     rule_owner_locator,
//   ).await {
//     Ok(rule) => {
//       return Ok(rule);
//     }
//     Err(error) => {
//       error
//     }
//   };

//   match error {
//     database::GetRuleError::InternalError => {
//       return Err(ActivateRuleReturn::InternalError);
//     }
//     database::GetRuleError::NoSuchOwner => {
//       return Err(ActivateRuleReturn::NoSuchOwner);
//     }
//     database::GetRuleError::NoSuchRule => {
//       return Err(ActivateRuleReturn::NoSuchRule);
//     }
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AddRule {
//   rule_creator: RuleCreator,
//   rule_owner_locator: RuleOwnerLocator,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum AddRuleReturn {
//   NoSuchOwner,
//   DuplicateUuid,
//   TooManyRules,
//   InternalError,
//   Success,
// }

// impl AddRule {
//   pub async fn execute(self, daemon: &Daemon) -> AddRuleReturn {
//     let mut info = daemon.state.rules.lock().await;
//     if info.rule_number >= info.maximum_rule_number {
//       return AddRuleReturn::TooManyRules;
//     }
    
//     let rule_id = self
//       .rule_creator
//       .id
//       .unwrap_or_else(UuidV4::generate);

//     let rule_activator = self
//       .rule_creator
//       .activator
//       .create();

//     let rule_enabler = self
//       .rule_creator
//       .enabler
//       .create();

//     if let Err(error) = database::add_rule(
//       &daemon.database,
//       &rule_id, 
//       &rule_activator,
//       &rule_enabler,
//       &self.rule_owner_locator,
//     ).await {
//       return match error {
//         database::AddRuleError::DuplicateId => {
//           AddRuleReturn::DuplicateUuid
//         }
//         database::AddRuleError::NoSuchOwner => {
//           AddRuleReturn::NoSuchOwner
//         }
//         database::AddRuleError::Other => {
//           AddRuleReturn::InternalError
//         }
//       };
//     }
    
//     cache::add_rule(
//       daemon, 
//       rule_id, 
//       rule_activator, 
//       rule_enabler, 
//       self.rule_owner_locator,
//     ).await;

//     info.rule_number += 1;

//     AddRuleReturn::Success
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DeleteRule {
//   rule_id: UuidV4,
//   // rule_owner_locator: RuleOwnerLocator,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeleteRuleReturn {
//   RuleStillEnabled,
//   NoSuchRule,
//   NoSuchOwner,
//   InternalError,
//   Success,
// }

// impl Into<DeleteRuleReturn> for database::GetRuleError1 {
//   fn into(self) -> DeleteRuleReturn {
//     match self {
//       Self::InternalError => DeleteRuleReturn::InternalError,
//       Self::NoSuchRule => DeleteRuleReturn::NoSuchRule,
//     }
//   }
// }

// impl DeleteRule {
//   pub async fn execute(self, daemon: &Daemon) -> DeleteRuleReturn {
//     let instant = daemon.state.clock.now();
    
//     let (rule, rule_owner_locator) = match database::get_rule_and_owner_locator(&daemon.database, &self.rule_id).await {
//       Ok(rule) => rule,
//       Err(error) => return error.into(),
//     };

//     if rule.is_enabled(instant) {
//       return DeleteRuleReturn::RuleStillEnabled;
//     }

//     let rule_owner_locator = database::get_rule(database, rule_id)

//     if let Err(error) = database::remove_rule(
//       &daemon.database,
//       &self.rule_id,
//       &self.rule_owner_locator,
//     ).await {
//       return match error {
//         database::RemoveRuleError::NoSuchRule => {
//           DeleteRuleReturn::NoSuchRule
//         }
//         database::RemoveRuleError::Other => {
//           DeleteRuleReturn::InternalError
//         }
//       };
//     }

//     cache::delete_rule(
//       daemon, 
//       &self.rule_id, 
//       &self.rule_owner_locator,
//     );
  
//     DeleteRuleReturn::Success
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ActivateRule {
//   rule_id: UuidV4,
//   rule_owner_locator: RuleOwnerLocator,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ActivateRuleReturn {
//   NoSuchRule,
//   NoSuchOwner,
//   InternalError,
//   Success,
// }

// impl ActivateRule {
//   pub async fn execute(self, daemon: &Daemon) -> ActivateRuleReturn {
//     let rule = cache::get_rule(daemon, &self.rule_id, &self.rule_owner_locator).await;

//     let rule = match get_rule(daemon, &self.rule_id, &self.rule_owner_locator).await {
//       Ok(rule) => {
//         rule
//       }
//       Err(error) => {
//         return error;
//       }
//     };

    
//     todo!()
//   }
// }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub struct DeactivateRule;

// // impl DeactivateRule {
// //   pub fn execute(
// //     self,
// //     rule: &mut Rule,
// //     now: MonotonicInstant,
// //   ) {
// //     rule.deactivate(now);
// //   }
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum RuleUpdate {
// //   Activate(ActivateRule),
// //   Deactivate(DeactivateRule),
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum RuleUpdateSuccess {
// //   Activate,
// //   Deactivate,
// // }

// // impl RuleUpdate {
// //   pub fn execute(
// //     self, 
// //     rule: &mut Rule,
// //     now: MonotonicInstant,
// //   ) -> RuleUpdateSuccess {
// //     match self {
// //       Self::Activate(proceduer) => {
// //         proceduer.execute(rule, now);
// //         RuleUpdateSuccess::Activate
// //       }
// //       Self::Deactivate(procedure) => {
// //         procedure.execute(rule, now);
// //         RuleUpdateSuccess::Deactivate
// //       }
// //     }
// //   }
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum Procedure {
// //   AddRule(AddRule),
// //   EnsureRuleDeleted(EnsureRuleDeleted),
// //   UpdateRule(UpdateRule),
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum ProcedureSuccess {
// //   AddRule,
// //   EnsureRuleDeleted,
// //   UpdateRule(UpdateRuleSuccess),
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum ProcedureFailure {
// //   AddRule(AddRuleFailure),
// //   EnsureRuleDeleted(EnsureRuleDeletedFailure),
// //   UpdateRule(UpdateRuleFailure),
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum ProcedureRevert {
// //   AddRule(RevertAddRule),
// //   EnsureRuleDeleted(Option<RevertDeleteRule>),
// //   UpdateRule(RevertUpdateRule),
// // }

// // impl Procedure {
// //   pub fn execute(
// //     self,
// //     transaction: &mut impl Transaction,
// //     writer: &impl TransactionWriter,
// //     rule_group: &mut RuleGroup,
// //     now: MonotonicInstant,
// //   ) -> Result<(ProcedureSuccess, ProcedureRevert), ProcedureFailure> {
// //     match self {
// //       Procedure::AddRule(inner) => {
// //         inner.execute(transaction, writer, rule_group) 
// //           .map(|revert| (
// //             ProcedureSuccess::AddRule,
// //             ProcedureRevert::AddRule(revert),
// //           ))
// //           .map_err(ProcedureFailure::AddRule)
// //       }
// //       Procedure::EnsureRuleDeleted(inner) => {
// //         inner.execute(transaction, writer, rule_group, now)
// //           .map(|revert| (
// //             ProcedureSuccess::EnsureRuleDeleted,
// //             ProcedureRevert::EnsureRuleDeleted(revert),
// //           ))
// //           .map_err(ProcedureFailure::EnsureRuleDeleted)
// //       }
// //       Procedure::UpdateRule(inner) => {
// //         inner.execute(transaction, writer, rule_group, now)
// //           .map(|(success, revert)| (
// //             ProcedureSuccess::UpdateRule(success),
// //             ProcedureRevert::UpdateRule(revert),
// //           ))
// //           .map_err(ProcedureFailure::UpdateRule)
// //       }
// //     }
// //   }
// // }



// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum AddRuleError {
// //   BadUuidV4TryAgain,
// //   ReachedMaximumRulesAllowedDeleteSomeAndTryAgain,
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum DeleteRuleSuccess {
// //   NoSuchRule,
// //   Success,
// // }

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // pub enum DeleteRuleError {
// //   RuleIsProtected,
// // }

// // impl CachedRuleGroup {
// //   pub fn add_rule_given_rule_creator(
// //     &mut self, 
// //     rule_creator: RuleCreator,
// //   ) -> Result<(UuidV4, Rule), AddRuleError> {
// //     if self.rules.len() >= self.maximum_rule_number {
// //       return Err(AddRuleError::ReachedMaximumRulesAllowedDeleteSomeAndTryAgain);
// //     }

// //     let id = rule_creator.id.unwrap_or_else(UuidV4::generate);
// //     if self.rules.contains_key(&id) {
// //       return Err(AddRuleError::BadUuidV4TryAgain);
// //     }

// //     let rule = Rule::new(
// //       rule_creator.activator.create(),
// //       rule_creator.enabler.create(),
// //     );

// //     self.rules.insert(id.clone(), rule.clone());
// //     Ok((id, rule))
// //   }

// //   pub fn delete_rule_given_rule_id(
// //     &mut self, 
// //     now: MonotonicInstant,
// //     rule_id: &UuidV4,
// //   ) -> Result<DeleteRuleSuccess, DeleteRuleError> {
// //     let Some(rule) = self.rules.get_mut(rule_id) else {
// //       return Ok(DeleteRuleSuccess::NoSuchRule);
// //     };

// //     if rule.is_enabled(now) {
// //       return Err(DeleteRuleError::RuleIsProtected);
// //     }

// //     self.rules.remove(rule_id);
// //     Ok(DeleteRuleSuccess::Success)
// //   }

// //   pub fn force_delete_rule_if_exists(&mut self, rule_id: &UuidV4) {
// //     self.rules.remove(rule_id);
// //   }
// // }

// // // pub mod procedures {
// // //   use crate::{rules::rules_x::{RuleEnablerX, RuleX}, x::{InstantX, RuleEnabler, countdown_after_plea_conditional_x, countdown_conditional, countdown_conditional_x}};

// // //   pub enum EnablerProcedure {
// // //     Countdown(countdown_conditional_x::procedures::Procedure),
// // //     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Procedure),
// // //   }

// // //   pub enum EnablerProcedureReturn {
// // //     VariantMismatch,
// // //     Countdown(countdown_conditional_x::procedures::Return),
// // //     CountdownAfterPlea(countdown_after_plea_conditional_x::procedures::Return),
// // //   }
  
// // //   impl EnablerProcedure {
// // //     pub fn execute(
// // //       self,
// // //       instant: InstantX,
// // //       conditional: &mut RuleEnablerX,
// // //     ) -> EnablerProcedureReturn {
// // //       match (self, conditional) {
// // //         (
// // //           EnablerProcedure::Countdown(operation), 
// // //           RuleEnablerX::Countdown(conditional)
// // //         ) => {
// // //           EnablerProcedureReturn::Countdown(
// // //             operation.execute(instant, conditional)
// // //           )
// // //         }
// // //         (
// // //           EnablerProcedure::CountdownAfterPlea(operation), 
// // //           RuleEnablerX::CountdownAfterPlea(conditional)
// // //         ) => {
// // //           EnablerProcedureReturn::CountdownAfterPlea(
// // //             operation.execute(instant, conditional)
// // //           )
// // //         }
// // //         _ => {
// // //           EnablerProcedureReturn::VariantMismatch
// // //         }
// // //       }
// // //     }
// // //   }

// // //   pub struct Activate {

// // //   }

// // //   pub trait DatabaseProcedures {
// // //     fn add_rule(&self, rule: &RuleX) -> Result<(), ()>;
// // //   }
  
// // //   pub struct CreateRule {
// // //     // rule_creator
// // //   }
// // // }


// // // // pub mod rules_x;

// // // // use std::collections::HashMap;
// // // // use serde::{Deserialize, Serialize};
// // // // use crate::x::{AlwaysConditional, CountdownAfterPleaConditional, CountdownConditional, DateTime, Time, TimeConditional, UuidV4, Weekday};

// // // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // // pub enum RuleActivator {
// // // //   Time(TimeConditional),
// // // //   Alwaus(AlwaysConditional),
// // // // }

// // // // impl RuleActivator {
// // // //   pub fn evaluate(&self, time: Time, weekday: Weekday) -> bool {
// // // //     match self {
// // // //       RuleActivator::Time(inner) => inner.evaulate(time, weekday),
// // // //       RuleActivator::Alwaus(inner) => inner.evaulate(),
// // // //     }
// // // //   }
// // // // }

// // // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // // pub enum RuleEnabler {
// // // //   Countdown(CountdownConditional),
// // // //   CountdownAfterPlea(CountdownAfterPleaConditional),
// // // // }

// // // // impl RuleEnabler {
// // // //   pub fn evaluate(&self) -> bool {
// // // //     match self {
// // // //       RuleEnabler::Countdown(inner) => inner.is_activated(),
// // // //       RuleEnabler::CountdownAfterPlea(inner) => inner.is_activated_or_deactivating(),
// // // //     }
// // // //   }

// // // //   pub fn synchronize(&mut self, now: DateTime) {
// // // //     match self {
// // // //       RuleEnabler::Countdown(inner) => inner.synchronize(now),
// // // //       RuleEnabler::CountdownAfterPlea(inner) => inner.synchronize(now),
// // // //     }
// // // //   }

// // // //   pub fn activate(&mut self, now: DateTime) {
// // // //     match self {
// // // //       RuleEnabler::Countdown(inner) => inner.activate(now),
// // // //       RuleEnabler::CountdownAfterPlea(inner) => inner.activate(),
// // // //     }
// // // //   }

// // // //   pub fn deactivate(&mut self, now: DateTime) {
// // // //     match self {
// // // //       RuleEnabler::Countdown(_inner) => {
// // // //         // noop
// // // //       }
// // // //       RuleEnabler::CountdownAfterPlea(inner) => {
// // // //         inner.deactivate(now);
// // // //       }
// // // //     }
// // // //   }
// // // // }

// // // // #[derive(Debug, Clone, Serialize, Deserialize)]
// // // // pub struct Rule {
// // // //   activator: RuleActivator,
// // // //   enabler: RuleEnabler,
// // // //   is_activated: bool,
// // // // }

// // // // impl Rule {
// // // //   pub fn new(
// // // //     activator: RuleActivator,
// // // //     enabler: RuleEnabler,
// // // //   ) -> Self {
// // // //     Self {
// // // //       activator,
// // // //       enabler,
// // // //       is_activated: false,
// // // //     }
// // // //   }

// // // //   pub fn construct(
// // // //     is_activated: bool,
// // // //     activator: RuleActivator,
// // // //     enabler: RuleEnabler,
// // // //   ) -> Self {
// // // //     Self {
// // // //       is_activated,
// // // //       activator,
// // // //       enabler,
// // // //     }
// // // //   }
  
// // // //   pub fn activator(&self) -> &RuleActivator {
// // // //     &self.activator
// // // //   }

// // // //   pub fn enabler(&self) -> &RuleEnabler {
// // // //     &self.enabler
// // // //   }

// // // //   pub fn is_activated(&self) -> bool {
// // // //     self.is_activated
// // // //   }

// // // //   pub fn is_effective(&self, time: Time, weekday: Weekday) -> bool {
// // // //     self.is_activated
// // // //     && 
// // // //     self.activator.evaluate(time, weekday)
// // // //   }

// // // //   pub fn is_protected(&self) -> bool {
// // // //     self.enabler.evaluate()
// // // //   }
// // // // }

// // // // pub struct RuleGroup {
// // // //   rules: HashMap<UuidV4, Rule>,
// // // //   maximum_rule_number: usize,
// // // // }

// // // // impl RuleGroup {
// // // //   pub fn new(maximum_rule_number: usize) -> Self {
// // // //     Self {
// // // //       rules: HashMap::new(),
// // // //       maximum_rule_number,
// // // //     }
// // // //   }
// // // // }

// // // // pub mod operations {
// // // //   use crate::x::{RuleEnabler, UuidV4, countdown_after_plea_conditional, countdown_conditional};

// // // //   // DaemonUsersRegulationBlockInternetAddRule
// // // //   // DaemonUsersRegulationBlockInternetDeleteRule
// // // //   // DaemonUsersRegulationBlockInternetActivateRule
// // // //   // DaemonUsersRegulationBlockInternetDeactivateRule

// // // //   // EnablerTypeMismatch
// // // //   // AlreadyActivated

// // // //   pub enum RuleEnablerActivate {
// // // //     Countdown(countdown_conditional::operations::Activate),
// // // //     CountdownAfterPlea(countdown_after_plea_conditional::operations::Activate),
// // // //   }

// // // //   pub enum RuleEnablerActivateReturn {
// // // //     TypeMismatch,
// // // //     Countdown(countdown_conditional::operations::ActivateReturn),
// // // //     CountdownAfterPlea(countdown_after_plea_conditional::operations::ActivateReturn),
// // // //   }
  
// // // //   impl RuleEnablerActivate {
// // // //     pub fn execute(
// // // //       self,
// // // //       conditional: &mut RuleEnabler,
// // // //     ) -> RuleEnablerActivateReturn {
// // // //       match (self, conditional) {
// // // //         (
// // // //           RuleEnablerActivate::Countdown(operation), 
// // // //           RuleEnabler::Countdown(conditional)
// // // //         ) => {
// // // //           RuleEnablerActivateReturn::Countdown(
// // // //             operation.execute(conditional)
// // // //           )
// // // //         }
// // // //         (
// // // //           RuleEnablerActivate::CountdownAfterPlea(operation), 
// // // //           RuleEnabler::CountdownAfterPlea(conditional)
// // // //         ) => {
// // // //           RuleEnablerActivateReturn::CountdownAfterPlea(
// // // //             operation.execute(conditional)
// // // //           )
// // // //         }
// // // //         _ => {
// // // //           RuleEnablerActivateReturn::TypeMismatch
// // // //         }
// // // //       }
// // // //     }
// // // //   }

// // // //   pub struct Activate {

// // // //   }
// // // // }
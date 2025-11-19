// use serde::{Deserialize, Serialize};
// use crate::x::database::Transaction;
// use crate::x::{Daemon, MonotonicInstant, UuidV4};
// use crate::x::rules_x::*;

// pub trait DatabaseProcedures {
//   async fn add_rule(&self, rule_id: &UuidV4, rule: &Rule) -> Result<(), ()>;
//   async fn delete_rule(&self, rule_id: &UuidV4) -> Result<(), ()>;
//   async fn update_rule(&self, rule_id: &UuidV4, original: &Rule, updated: &Rule) -> Result<(), ()>;
//   async fn update_rule_enabler(&self, rule_id: &UuidV4, original: &RuleEnabler, updated: &RuleEnabler) -> Result<(), ()>;
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AddRule {
//   rule_creator: RuleCreator,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum AddRuleReturn {
//   DuplicateUuid,
//   TooManyRules,
//   InternalError,
//   Success,
// }

// impl AddRule {
//   pub async fn execute(
//     self, 
//     database: &mut impl DatabaseProcedures,
//     rule_group: &mut RuleGroup,
//   ) -> AddRuleReturn {
//     if rule_group.rules.len() >= rule_group.maximum_rule_number {
//       return AddRuleReturn::TooManyRules;
//     }
    
//     let rule_id = self.rule_creator.id.unwrap_or_else(UuidV4::generate);
//     if rule_group.rules.contains_key(&rule_id) {
//       return AddRuleReturn::DuplicateUuid;
//     }

//     let rule = Rule::new(
//       self.rule_creator.activator.create(),
//       self.rule_creator.enabler.create(),
//     );

//     if let Err(_) = database.add_rule(&rule_id, &rule).await {
//       return AddRuleReturn::InternalError;
//     }

//     rule_group.rules.insert(rule_id.clone(), rule);

//     AddRuleReturn::Success
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DeleteRule {
//   rule_id: UuidV4,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeleteRuleReturn {
//   RuleIsProtected,
//   NoSuchRule,
//   InternalError,
//   Success,
// }

// impl DeleteRule {
//   pub async fn execute(
//     self,
//     database: &impl DatabaseProcedures,
//     rule_group: &mut RuleGroup,
//     now: MonotonicInstant,
//   ) -> DeleteRuleReturn {
//     let Some(rule) = rule_group.rules.get(&self.rule_id) else {
//       return DeleteRuleReturn::NoSuchRule;
//     };

//     if rule.is_enabled(now) {
//       return DeleteRuleReturn::RuleIsProtected;
//     }

//     if let Err(_) = database.delete_rule(&self.rule_id).await {
//       return DeleteRuleReturn::Success
//     }

//     rule_group.rules.remove(&self.rule_id);
  
//     DeleteRuleReturn::Success
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EnableRule {
//   rule_id: UuidV4,
// }

// pub enum EnableRuleReturn {
//   NoSuchRule,
//   InternalError,
//   Success,
// }

// impl EnableRule {
//   pub async fn execute(
//     self,
//     database: &mut impl DatabaseProcedures,
//     rule_group: &mut RuleGroup,
//     now: MonotonicInstant,
//   ) -> EnableRuleReturn {
//     let Some(rule) = rule_group.rules.get_mut(&self.rule_id) else {
//       return EnableRuleReturn::NoSuchRule;
//     };

//     let original_enabler = rule.enabler().clone();
//     rule.enabler_mut().activate(now);

//     if let Err(_) = database.update_rule_enabler(
//       &self.rule_id, 
//       &original_enabler, 
//       rule.enabler(),
//     ).await {
//       rule.set_enabler(original_enabler);
//       return EnableRuleReturn::InternalError;
//     }

//     EnableRuleReturn::Success
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Disable {
//   rule_id: UuidV4,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DisableReturn {
//   NoSuchRule,
//   InternalError,
//   Success,
// }

// impl Disable {
//   pub async fn execute(
//     self,
//     database: &mut impl DatabaseProcedures,
//     rule_group: &mut RuleGroup,
//     now: MonotonicInstant,
//   ) -> DisableReturn {
//     let Some(rule) = rule_group.rules.get_mut(&self.rule_id) else {
//       return DisableReturn::NoSuchRule;
//     };

//     let original_enabler = rule.enabler().clone();
//     rule.enabler_mut().deactivate(now);

//     if let Err(_) = database
//       .update_rule_enabler(&self.rule_id, &original_enabler, rule.enabler()).await 
//     {
//       rule.set_enabler(original_enabler);
//     }
//   }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum RuleUpdate {
//   Activate(EnableRule),
//   Deactivate(Disable),
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
//   EnsureRuleDeleted(DeleteRule),
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
//   AddRule(AddRuleReturn),
//   EnsureRuleDeleted(DeleteRuleReturn),
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
//     writer: &impl DatabaseProcedures,
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

// pub enum StandAloneProcedure {
//   UserBlockDeviceAccessRuleGroupRules { 
//     user_id: UuidV4,
//     procedure: Procedure,
//   },
//   UserBlockLoginAccessRuleGroupRules { 
//     user_id: UuidV4,
//     procedure: Procedure,
//   },
//   UserBlockInternetAccessRuleGroupRules { 
//     user_id: UuidV4,
//     procedure: Procedure,
//   },
// }

// pub enum StandAloneProcedureError {
//   NoSuchUser,
// }


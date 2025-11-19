use crate::x::UuidV4;

pub struct CachedRule {
  
}

pub enum RuleOwnerLocator {
  UserDeviceAccessRegulation {
    user_id: UuidV4,
  },
  UserInternetAccessRegulation {
    user_id: UuidV4,
  },
  UserLoginAccessRegulation {
    user_id: UuidV4,
  },
}

pub struct FullRule {
  
}

pub struct Rules {}

pub struct CrossRuleInfo {}

pub struct ScreenAccessRegulation {

}

use serde::{Deserialize, Serialize};
use crate::x::database::Transaction;
use crate::x::{Daemon, MonotonicInstant, UuidV4};
use crate::x::rules_x::*;

pub enum RuleLocation {
  UserLoginAccessRegulation {
    user_id: UuidV4,
  },
  UserDeviceAccessRegulation {
    user_id: UuidV4,
  },
  UserInternetAccessRegulation {
    user_id: UuidV4,
  },
}

pub struct RuleCollection {

}

pub enum DatabaseError {
  PrimaryKeyViolation,
  Unknown,
}

impl RuleCollection {
  pub fn add_rule(
    &self, 
    rule_id: &UuidV4, 
    rule_location: &RuleLocation, 
    rule: &Rule,
  ) -> Result<(), DatabaseError> {

  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  rule_creator: RuleCreator,
  rule_location: RuleLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleFailure {
  DuplicateUuid,
  TooManyRules,
  PrimaryKeyViolation,
  UserIdForeignKeyVioation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertAddRule {
  rule_id: UuidV4,
}

impl AddRule {
  pub fn execute(self, daemon: &Daemon) -> Result<RevertAddRule, AddRuleFailure> {
    let info = &mut daemon.state.rules;

    if info.rule_number >= info.maximum_rule_number {
      return Err(AddRuleFailure::TooManyRules);
    }
    
    let rule_id = self.rule_creator.id.unwrap_or_else(UuidV4::generate);

    let rule = Rule::new(
      self.rule_creator.activator.create(),
      self.rule_creator.enabler.create(),
    );

    // check whether foreign key constraint is violated
    // check whether primary key constraint is violated

    writer.add_rule(transaction, &rule_id, &rule);

    rule_group.rules.insert(rule_id.clone(), rule);
    
    Ok(RevertAddRule { rule_id })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureRuleDeleted {
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnsureRuleDeletedFailure {
  RuleIsProtected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertDeleteRule {
  rule: Rule,
  rule_id: UuidV4,
}

impl EnsureRuleDeleted {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    writer: &impl TransactionWriter,
    rule_group: &mut RuleGroup,
    now: MonotonicInstant,
  ) -> Result<Option<RevertDeleteRule>, EnsureRuleDeletedFailure> {
    let Some(rule) = rule_group.rules.get(&self.rule_id) else {
      return Ok(None);
    };

    if rule.is_enabled(now) {
      return Err(EnsureRuleDeletedFailure::RuleIsProtected);
    }

    writer.delete_rule(transaction, &self.rule_id);

    // SAFETY: We are sure the rule exists: We accessed it above.
    //         This unwrap won't fail.
    let rule = unsafe {
      rule_group.rules.remove(&self.rule_id).unwrap_unchecked()
    };
  
    Ok(Some(RevertDeleteRule { 
      rule, 
      rule_id: self.rule_id,
    }))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRule {
  rule_id: UuidV4,
  updates: Vec<RuleUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateRuleFailure {
  NoSuchRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRuleSuccess {
  successes: Vec<RuleUpdateSuccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertUpdateRule {
  rule_id: UuidV4,
  rule_original_state: Rule,
}

impl UpdateRule {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    writer: &impl TransactionWriter,
    rule_group: &mut RuleGroup,
    now: MonotonicInstant,
  ) -> Result<(UpdateRuleSuccess, RevertUpdateRule), UpdateRuleFailure> {
    let rule_id = self.rule_id;

    let Some(rule) = rule_group.rules.get_mut(&rule_id) else {
      return Err(UpdateRuleFailure::NoSuchRule);
    };

    let rule_original_state = rule.clone();

    let successes = self
      .updates
      .into_iter()
      .map(|it| it.execute(rule, now))
      .collect();

    writer.update_rule(
      transaction, 
      &rule_id, 
      &rule_original_state, 
      &rule,
    );

    Ok((
      UpdateRuleSuccess { successes },
      RevertUpdateRule { rule_id, rule_original_state }
    ))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateRule;

impl ActivateRule {
  pub fn execute(
    self,
    rule: &mut Rule,
    now: MonotonicInstant,
  ) {
    rule.activate(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivateRule;

impl DeactivateRule {
  pub fn execute(
    self,
    rule: &mut Rule,
    now: MonotonicInstant,
  ) {
    rule.deactivate(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleUpdate {
  Activate(ActivateRule),
  Deactivate(DeactivateRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleUpdateSuccess {
  Activate,
  Deactivate,
}

impl RuleUpdate {
  pub fn execute(
    self, 
    rule: &mut Rule,
    now: MonotonicInstant,
  ) -> RuleUpdateSuccess {
    match self {
      Self::Activate(proceduer) => {
        proceduer.execute(rule, now);
        RuleUpdateSuccess::Activate
      }
      Self::Deactivate(procedure) => {
        procedure.execute(rule, now);
        RuleUpdateSuccess::Deactivate
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Procedure {
  AddRule(AddRule),
  EnsureRuleDeleted(EnsureRuleDeleted),
  UpdateRule(UpdateRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcedureSuccess {
  AddRule,
  EnsureRuleDeleted,
  UpdateRule(UpdateRuleSuccess),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcedureFailure {
  AddRule(AddRuleFailure),
  EnsureRuleDeleted(EnsureRuleDeletedFailure),
  UpdateRule(UpdateRuleFailure),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcedureRevert {
  AddRule(RevertAddRule),
  EnsureRuleDeleted(Option<RevertDeleteRule>),
  UpdateRule(RevertUpdateRule),
}

impl Procedure {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    writer: &impl TransactionWriter,
    rule_group: &mut RuleGroup,
    now: MonotonicInstant,
  ) -> Result<(ProcedureSuccess, ProcedureRevert), ProcedureFailure> {
    match self {
      Procedure::AddRule(inner) => {
        inner.execute(transaction, writer, rule_group) 
          .map(|revert| (
            ProcedureSuccess::AddRule,
            ProcedureRevert::AddRule(revert),
          ))
          .map_err(ProcedureFailure::AddRule)
      }
      Procedure::EnsureRuleDeleted(inner) => {
        inner.execute(transaction, writer, rule_group, now)
          .map(|revert| (
            ProcedureSuccess::EnsureRuleDeleted,
            ProcedureRevert::EnsureRuleDeleted(revert),
          ))
          .map_err(ProcedureFailure::EnsureRuleDeleted)
      }
      Procedure::UpdateRule(inner) => {
        inner.execute(transaction, writer, rule_group, now)
          .map(|(success, revert)| (
            ProcedureSuccess::UpdateRule(success),
            ProcedureRevert::UpdateRule(revert),
          ))
          .map_err(ProcedureFailure::UpdateRule)
      }
    }
  }
}
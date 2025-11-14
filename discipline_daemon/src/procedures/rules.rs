use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{database::SqlCode, x::{Daemon, InstantX, Rule, UuidV4, rules_x::{self, RuleX, RuleGroupX}}};

pub trait Transaction {}

pub trait Writer {
  fn add_rule(
    &self, 
    transaction: &mut impl Transaction, 
    rule_id: &UuidV4, 
    rule: &RuleX,
  );

  fn delete_rule(
    &self,
    transaction: &mut impl Transaction,
    rule_id: &UuidV4,
  );

  fn update_rule(
    &self,
    transaction: &mut impl Transaction,
    rule_id: &UuidV4,
    original: &RuleX,
    modified: &RuleX,
  );
}

pub struct CrossCollectionDraft {
  code: SqlCode
}

pub struct CollectionItemDraft {
  code: SqlCode
}

pub trait RuleGroupWriter {
  fn add_rule(
    &self, 
    draft: &mut CrossCollectionDraft, 
    rule_id: &UuidV4, 
    rule: &RuleX,
  );

  fn delete_rule(
    &self,
    draft: &mut CrossCollectionDraft,
    rule_id: &UuidV4,
  );

  fn update_rule(
    &self,
    rule_id: &UuidV4,
    original: &RuleX,
    modified: &RuleX,
  );
}

pub trait RuleWriter {
  
}

pub enum Tried<T, E, R> {
  Ok(T),
  OkWithRevert(T, R),
  Revert(R),
  Err(E),
}

impl<T, E, R> Tried<T, E, R> {
  pub fn ok(value: T) -> Self {
    Tried::Ok(value)
  }

  pub fn err(error: E) -> Self {
    Tried::Err(error)
  }

  pub fn ok_with_revert(value: T, revert: R) -> Self {
    Tried::OkWithRevert(value, revert)
  }

  pub fn revert(revert: R) -> Self {
    Tried::Revert(revert)
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  rule_creator: rules_x::RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleError {
  DuplicateUuid,
  TooManyRules,
}

pub struct RevertAddRule {
  rule_id: UuidV4,
}

impl AddRule {
  pub fn execute(
    self, 
    transaction: &mut CrossCollectionDraft,
    writer: &impl RuleGroupWriter,
    rule_group: &mut rules_x::RuleGroupX,
  ) -> Tried<(), AddRuleError, RevertAddRule> {
    if rule_group.rules.len() >= rule_group.maximum_rule_number {
      return Tried::Err(AddRuleError::TooManyRules);
    }
    
    let rule_id = self.rule_creator.id.unwrap_or_else(UuidV4::generate);
    if rule_group.rules.contains_key(&rule_id) {
      return Tried::Err(AddRuleError::DuplicateUuid);
    }

    let rule = rules_x::RuleX::new(
      self.rule_creator.action_conditional.create(),
      self.rule_creator.protection_conditional.create(),
    );

    writer.add_rule(transaction, &rule_id, &rule);

    rule_group.rules.insert(rule_id.clone(), rule);
    
    Tried::revert(RevertAddRule { rule_id })
  }
}

pub struct EnsureRuleDeleted {
  rule_id: UuidV4,
}

pub enum EnsureRuleDeletedError {
  RuleIsProtected,
}

pub struct RevertDeleteRule {
  rule: RuleX,
  rule_id: UuidV4,
}

impl EnsureRuleDeleted {
  pub fn execute(
    self,
    transaction: &mut CrossCollectionDraft,
    writer: &impl RuleGroupWriter,
    rule_group: &mut RuleGroupX,
    now: InstantX,
  ) -> Tried<(), EnsureRuleDeletedError, RevertDeleteRule> {
    let Some(rule) = rule_group.rules.get(&self.rule_id) else {
      return Tried::ok(());
    };

    if rule.is_protected(now) {
      return Tried::err(EnsureRuleDeletedError::RuleIsProtected);
    }

    writer.delete_rule(transaction, &self.rule_id);

    // SAFETY: We are sure the rule exists: We accessed it above.
    //         This unwrap won't fail.
    let rule = unsafe {
      rule_group.rules.remove(&self.rule_id).unwrap_unchecked()
    };
  
    Tried::revert(RevertDeleteRule { 
      rule, 
      rule_id: self.rule_id,
    })
  }
}

pub struct ModifyRule {
  rule_id: UuidV4,
  modifications: Vec<RuleProcedure>,
}

pub enum ModifyRuleError {
  NoSuchRule,
  RuleModificationError(RuleModificationFailure)
}

pub struct ModifyRuleSuccess {
  successes: Vec<RuleModificationSuccess>,
}

pub struct ModifyRuleRevert {
  rule_id: UuidV4,
  rule_original_state: RuleX,
}

impl ModifyRule {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    writer: &impl Writer,
    rule_group: &mut rules_x::RuleGroupX,
  ) -> Result<(ModifyRuleSuccess, ModifyRuleRevert), ModifyRuleError> {
    let Some(rule) = rule_group.rules.get_mut(&self.rule_id) else {
      return Err(ModifyRuleError::NoSuchRule);
    };

    let mut rule_original_state = rule.clone();

    let mut successes = Vec::new();
    for modification in self.modifications {
      match modification.execute(&mut rule_original_state) {
        Ok(value) => {
          successes.push(value);
        }
        Err(error) => {
          return Err(ModifyRuleError::RuleModificationError(error));
        }
      }
    }

    writer.update_rule(transaction, &self.rule_id, &rule_original_state, &rule);

    Ok((
      ModifyRuleSuccess { successes },
      ModifyRuleRevert { rule_id: self.rule_id, rule_original_state }
    ))
  }
}

pub struct ActivateRule;

impl ActivateRule {
  pub fn execute(
    self,
    rule: &mut RuleX,
    now: InstantX,
  ) {
    rule.activate(now);
  }
}

pub struct DeactivateRule {

}

pub enum RuleProcedure {
  Activate(ActivateRule),
  Deactivate(DeactivateRule),
}

pub struct RuleModificationSuccess {}
pub struct RuleModificationFailure {}

impl RuleProcedure {
  pub fn execute(self, rule: &mut RuleX) -> 
    Result<RuleModificationSuccess, RuleModificationFailure> 
  {
    todo!()
  }
}
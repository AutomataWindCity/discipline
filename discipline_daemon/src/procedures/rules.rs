use serde::{Deserialize, Serialize};
use crate::x::database::Transaction;
use crate::x::{MonotonicInstant, UuidV4};
use crate::x::rules_x::*;

pub trait TransactionWriter {
  fn add_rule(
    &self, 
    transaction: &mut impl Transaction, 
    rule_id: &UuidV4, 
    rule: &Rule,
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
    original: &Rule,
    modified: &Rule,
  );
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  rule_creator: RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleError {
  DuplicateUuid,
  TooManyRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertAddRule {
  rule_id: UuidV4,
}

impl AddRule {
  pub fn execute(
    self, 
    transaction: &mut impl Transaction,
    writer: &impl TransactionWriter,
    rule_group: &mut RuleGroup,
  ) -> Result<RevertAddRule, AddRuleError> {
    if rule_group.rules.len() >= rule_group.maximum_rule_number {
      return Err(AddRuleError::TooManyRules);
    }
    
    let rule_id = self.rule_creator.id.unwrap_or_else(UuidV4::generate);
    if rule_group.rules.contains_key(&rule_id) {
      return Err(AddRuleError::DuplicateUuid);
    }

    let rule = Rule::new(
      self.rule_creator.activator.create(),
      self.rule_creator.enabler.create(),
    );

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
pub enum EnsureRuleDeletedError {
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
  ) -> Result<Option<RevertDeleteRule>, EnsureRuleDeletedError> {
    let Some(rule) = rule_group.rules.get(&self.rule_id) else {
      return Ok(None);
    };

    if rule.is_protected(now) {
      return Err(EnsureRuleDeletedError::RuleIsProtected);
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
pub struct ModifyRule {
  rule_id: UuidV4,
  modifications: Vec<RuleModification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifyRuleFailure {
  NoSuchRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleSuccess {
  successes: Vec<RuleModificationSuccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleRevert {
  rule_id: UuidV4,
  rule_original_state: Rule,
}

impl ModifyRule {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    transaction_writer: &impl TransactionWriter,
    rule_group: &mut RuleGroup,
    now: MonotonicInstant,
  ) -> Result<(ModifyRuleSuccess, ModifyRuleRevert), ModifyRuleFailure> {
    let rule_id = self.rule_id;

    let Some(rule) = rule_group.rules.get_mut(&rule_id) else {
      return Err(ModifyRuleFailure::NoSuchRule);
    };

    let rule_original_state = rule.clone();

    let successes = self
      .modifications
      .into_iter()
      .map(|it| it.execute(rule, now))
      .collect();

    transaction_writer.update_rule(
      transaction, 
      &rule_id, 
      &rule_original_state, 
      &rule,
    );

    Ok((
      ModifyRuleSuccess { successes },
      ModifyRuleRevert { rule_id, rule_original_state }
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
pub enum RuleModification {
  Activate(ActivateRule),
  Deactivate(DeactivateRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleModificationSuccess {
  Activate,
  Deactivate,
}

impl RuleModification {
  pub fn execute(
    self, 
    rule: &mut Rule,
    now: MonotonicInstant,
  ) -> RuleModificationSuccess {
    match self {
      Self::Activate(proceduer) => {
        proceduer.execute(rule, now);
        RuleModificationSuccess::Activate
      }
      Self::Deactivate(procedure) => {
        procedure.execute(rule, now);
        RuleModificationSuccess::Deactivate
      }
    }
  }
}
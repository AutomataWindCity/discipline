use serde::{Deserialize, Serialize};
use crate::x::{BlockDeviceAccess, MonotonicInstant};
use crate::x::database::Transaction;

pub trait TransactionWriter {
  fn rule_group(&self) -> &impl super::rule_group_rules::TransactionWriter;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleGroup {
  procedures: Vec<super::rule_group_rules::Procedure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleGroupSuccess {
  successes: Vec<super::rule_group_rules::ProcedureSuccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleGroupFailure {
  failure: super::rule_group_rules::ProcedureFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleGroupRevert {
  reverts: Vec<super::rule_group_rules::ProcedureRevert>,
}

impl ModifyRuleGroup {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    writer: &impl TransactionWriter,
    regulation: &mut BlockDeviceAccess,
    now: MonotonicInstant,
  ) -> Result<(ModifyRuleGroupSuccess, ModifyRuleGroupRevert), ModifyRuleGroupFailure> {
    let rule_group_writer = writer.rule_group();
    let rule_group = &mut regulation.rules;

    let mut successes = Vec::new();
    let mut reverts = Vec::new();

    for procedure in self.procedures {
      match procedure.execute(
        transaction, 
        rule_group_writer,
        rule_group, 
        now
      ) {
        Ok((success, revert)) => {
          successes.push(success);
          reverts.push(revert);
        }
        Err(failure) => {
          return Err(ModifyRuleGroupFailure { failure });
        }
      }
    }

    Ok((
      ModifyRuleGroupSuccess { successes },
      ModifyRuleGroupRevert { reverts }
    ))
  }
}

pub enum Procedure {}

pub enum ProcedureReturn {}
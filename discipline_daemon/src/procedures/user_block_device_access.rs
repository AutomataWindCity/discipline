use serde::{Deserialize, Serialize};
use crate::x::{UuidV4, rules::rules_x};
use crate::x::database::user_rules::UserRuleCollectionProcedures;
use crate::x::regulation::block_device_access::BlockDeviceAccess;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  user_id: UuidV4,
  rule_creator: rules_x::RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleReturn {
  InternalError,
  BadUuidV4TryAgain,
  ReachedMaximumRulesAllowedDeleteSomeAndTryAgain,
  Success,
}

impl AddRule {
  pub fn execute(
    self,
    collection: &mut UserRuleCollectionProcedures,
    regulation: &mut BlockDeviceAccess,
  ) {
    let x = match regulation.rules.add_rule_given_rule_creator(self.rule_creator) {
      Ok()
    };

    // collection.add_rule(rule, rule_id, user_id)
  }
}



pub struct DeleteRule {

}

pub struct ActivateRule {

}

pub struct DeactivateRule {

}
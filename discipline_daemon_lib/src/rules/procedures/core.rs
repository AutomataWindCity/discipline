use serde::{Serialize, Deserialize};
use crate::x::{Database, MonotonicInstant, UuidV4};
use crate::x::rules::{Rule, RuleGroup, RuleActivatorCreator, RuleEnablerCreator, CrossGroupInfo, Location};
use crate::x::rules::database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleReturn {
  TooManyRules,
  DuplicateId,
  InternalError,
  Success,
}

pub async fn add_rule(
  database: &Database,
  location: &Location,
  rule_group: &mut RuleGroup,
  cross_group_info: &mut CrossGroupInfo,
  rule_id: Option<UuidV4>,
  rule_activator_creator: RuleActivatorCreator,
  rule_enabler_creator: RuleEnablerCreator,
) -> AddRuleReturn {
  if cross_group_info.reached_maximum_rule_number() {
    return AddRuleReturn::TooManyRules;
  }

  let rule_id_is_created_by_client = rule_id.is_some();
  let rule_id = rule_id.unwrap_or_else(UuidV4::generate);
  let rule_activator = rule_activator_creator.create();
  let rule_enabler = rule_enabler_creator.create();

  if let Err(error) = database::add_rule(
    database,
    location,
    &rule_id, 
    &rule_enabler, 
    &rule_activator, 
  ).await {
    return match error {
      database::AddRuleError::DuplicateId => {
        if rule_id_is_created_by_client {
          AddRuleReturn::DuplicateId
        } else {
          AddRuleReturn::InternalError
        }
      }
      database::AddRuleError::Other => {
        AddRuleReturn::InternalError
      }
    };
  }

  let rule = Rule::new(rule_activator, rule_enabler);
  rule_group.add_rule(rule_id, rule);
  cross_group_info.increment_rule_number();

  AddRuleReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  NoSuchRule,
  RuleStillEnabled,
  InternalError,
  Success,
}

pub async fn delete_rule(
  database: &Database,
  location: &Location,
  rule_group: &mut RuleGroup,
  cross_group_info: &mut CrossGroupInfo,
  rule_id: &UuidV4,
  now: MonotonicInstant,
) -> DeleteRuleReturn {
  let Some(rule) = rule_group.get_rule_mut(rule_id) else {
    return DeleteRuleReturn::NoSuchRule;
  };

  if rule.is_enabled(now) {
    return DeleteRuleReturn::RuleStillEnabled;
  }

  if let Err(error) = database::delete_rule(
    database,
    location,
    rule_id,
  ).await {
    return match error {
      database::DeleteRuleError::InternalError => DeleteRuleReturn::InternalError,
      database::DeleteRuleError::NoSuchRule => DeleteRuleReturn::InternalError,
    };
  }

  rule_group.delete_rule(rule_id);
  cross_group_info.decrement_rule_number();

  DeleteRuleReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivateRuleReturn {
  NoSuchRule,
  InternalError,
  Success,
}

pub async fn activate_rule(
  database: &Database,
  location: &Location,
  rule_group: &mut RuleGroup,
  rule_id: &UuidV4,
  now: MonotonicInstant,
) -> ActivateRuleReturn {
  let Some(rule) = rule_group.get_rule_mut(rule_id) else {
    return ActivateRuleReturn::NoSuchRule;
  };

  let mut enabler = rule.enabler.clone();
  enabler.activate(now);

  if let Err(error) = database::update_rule_enabler(
    database,
    location,
    rule_id,
    &rule.enabler, 
    &enabler,
  ).await {
    return match error {
      database::UpdateRuleEnablerError::InternalError => ActivateRuleReturn::InternalError,
      database::UpdateRuleEnablerError::NoSuchRule => ActivateRuleReturn::InternalError,
    };
  }

  rule.enabler = enabler;
  ActivateRuleReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeactivateRuleReturn {
  NoSuchRule,
  InternalError,
  Success,
}

pub async fn deactivate_rule(
  database: &Database,
  location: &Location,
  rule_group: &mut RuleGroup,
  rule_id: &UuidV4,
  now: MonotonicInstant,
) -> DeactivateRuleReturn {
  let Some(rule) = rule_group.get_rule_mut(rule_id) else {
    return DeactivateRuleReturn::NoSuchRule;
  };

  let mut enabler = rule.enabler.clone();
  enabler.deactivate(now);

  if let Err(error) = database::update_rule_enabler(
    database, 
    location, 
    rule_id, 
    &rule.enabler, 
    &enabler,
  ).await {
    return match error {
      database::UpdateRuleEnablerError::InternalError => DeactivateRuleReturn::InternalError,
      database::UpdateRuleEnablerError::NoSuchRule => DeactivateRuleReturn::InternalError,
    };
  }

  rule.enabler = enabler;
  DeactivateRuleReturn::Success
}

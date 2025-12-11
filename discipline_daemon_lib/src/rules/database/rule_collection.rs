use crate::x::{UuidV4, Database};
use crate::x::database::*;
use crate::x::rules::{RuleActivator, RuleEnabler, Location};
use crate::x::rules::database::user_rule_collection;

pub enum AddRuleError {
  DuplicateId,
  Other,
}

pub async fn add_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
  rule_enabler: &RuleEnabler, 
  rule_activator: &RuleActivator, 
) -> Result<(), AddRuleError> {
  let maybe_error = match location {
    Location::UserAccountAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
    Location::UserDeviceAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
    Location::UserInternetAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_internet_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
  };

  let Err(error) = maybe_error else {
    return Ok(());
  };

  Err(match error {
    DbExecuteError::PrimaryKeyViolation => AddRuleError::DuplicateId,
    DbExecuteError::ForiegnKeyViolation => AddRuleError::Other,
    DbExecuteError::Other(it) => AddRuleError::Other,
  })
}

pub enum RemoveRuleError {
  NoSuchRule,
  InternalError,
}

pub async fn remove_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
) -> Result<(), RemoveRuleError> {
  let maybe_error = match location {
    Location::UserDeviceAccessRegulation { .. } => {
      user_rule_collection::remove_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_collection, 
        rule_id,
      ).await
    }
    Location::UserAccountAccessRegulation { .. } => {
      user_rule_collection::remove_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_collection, 
        rule_id,
      ).await
    }
    Location::UserInternetAccessRegulation { .. } => {
      user_rule_collection::remove_rule(
        &database.connection, 
        &database.user_internet_access_regulation_rule_collection,
        rule_id,
      ).await
    }
  };

  let Err(error) = maybe_error else {
    return Ok(());
  };

  Err(match error {
    DbExecuteError::ForiegnKeyViolation => RemoveRuleError::InternalError,
    DbExecuteError::Other(it) => RemoveRuleError::InternalError,
    DbExecuteError::PrimaryKeyViolation => RemoveRuleError::InternalError,
  })
}

pub enum UpdateRuleEnablerError {
  NoSuchRule,
  InternalError,
}

pub async fn update_rule_enabler(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
  original_enabler: &RuleEnabler,
  new_enabler: &RuleEnabler,
) -> Result<(), UpdateRuleEnablerError> {
  todo!();
}

pub enum DeleteRuleError {
  InternalError,
  NoSuchRule,
}

pub async fn delete_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
) -> Result<(), DeleteRuleError> {
  todo!()
}
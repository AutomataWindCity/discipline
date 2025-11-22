use crate::x::{UuidV4, Database};
use crate::x::database::*;
use crate::x::rules::{Rule, RuleActivator, RuleEnabler, Location};
use crate::x::rules::database::user_rule_collection;

pub enum DbAddRuleError {
  DuplicateId,
  InternalError,
}

pub async fn db_add_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
  rule_activator: &RuleActivator, 
  rule_enabler: &RuleEnabler, 
) -> Result<(), DbAddRuleError> {
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
    DbExecuteError::PrimaryKeyViolation => DbAddRuleError::DuplicateId,
    DbExecuteError::ForiegnKeyViolation => DbAddRuleError::InternalError,
    DbExecuteError::Other(it) => DbAddRuleError::InternalError,
  })
}

pub enum DbRemoveRuleError {
  NoSuchRule,
  InternalError,
}

pub async fn remove_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
) -> Result<(), DbRemoveRuleError> {
  let maybe_error = match location {
    Location::UserDeviceAccessRegulation { user_id } => {
      user_rule_collection::remove_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_collection, 
        rule_id,
      ).await
    }
    Location::UserAccountAccessRegulation { user_id } => {
      user_rule_collection::remove_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_collection, 
        rule_id,
      ).await
    }
    Location::UserInternetAccessRegulation { user_id } => {
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
    DbExecuteError::ForiegnKeyViolation => DbRemoveRuleError::InternalError,
    DbExecuteError::Other(it) => DbRemoveRuleError::InternalError,
    DbExecuteError::PrimaryKeyViolation => DbRemoveRuleError::InternalError,
  })
}

pub enum DbUpdateRuleEnablerError {
  NoSuchRule,
  InternalError,
}

pub async fn db_update_rule_enabler(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
  original_enabler: &RuleEnabler,
  new_enabler: &RuleEnabler,
) -> Result<(), DbUpdateRuleEnablerError> {
  
}
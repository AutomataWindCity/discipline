// TODO: Do better error handling and logging

use crate::x::{UuidV4, Database, TextualError};
use crate::x::database::*;
use crate::x::rules::{Rule, RuleEnabler, Location};
use crate::x::database::rules::user_rule_table as table;

pub use table::Row;

pub enum InsertRuleError {
  DuplicateId,
  Other,
}

pub async fn insert_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
  rule: &Rule,
) -> Result<(), InsertRuleError> {
  let maybe_error = match location {
    Location::UserAccountAccessRegulation { user_id } => {
      table::insert_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_table, 
        rule, 
        rule_id, 
        user_id,
      ).await
    }
    Location::UserDeviceAccessRegulation { user_id } => {
      table::insert_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        rule, 
        rule_id, 
        user_id,
      ).await
    }
    Location::UserInternetAccessRegulation { user_id } => {
      table::insert_rule(
        &database.connection, 
        &database.user_internet_access_regulation_rule_table, 
        rule, 
        rule_id, 
        user_id,
      ).await
    }
  };

  let Err(error) = maybe_error else {
    return Ok(());
  };

  Err(match error {
    DbExecuteError::PrimaryKeyViolation => InsertRuleError::DuplicateId,
    DbExecuteError::ForiegnKeyViolation => InsertRuleError::Other,
    DbExecuteError::Other(it) => InsertRuleError::Other,
  })
}

pub enum DeleteRuleError {
  NoSuchRule,
  InternalError,
}

pub async fn delete_rule(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4, 
) -> Result<(), DeleteRuleError> {
  let maybe_error = match location {
    Location::UserDeviceAccessRegulation { .. } => {
      table::delete_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        rule_id,
      ).await
    }
    Location::UserAccountAccessRegulation { .. } => {
      table::delete_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_table, 
        rule_id,
      ).await
    }
    Location::UserInternetAccessRegulation { .. } => {
      table::delete_rule(
        &database.connection, 
        &database.user_internet_access_regulation_rule_table,
        rule_id,
      ).await
    }
  };

  let Err(error) = maybe_error else {
    return Ok(());
  };

  Err(match error {
    DbExecuteError::ForiegnKeyViolation => DeleteRuleError::InternalError,
    DbExecuteError::Other(it) => DeleteRuleError::InternalError,
    DbExecuteError::PrimaryKeyViolation => DeleteRuleError::InternalError,
  })
}

pub enum SetRuleEnablerError {
  NoSuchRule,
  Other,
}

pub async fn set_rule_enabler(
  database: &Database,
  location: &Location,
  rule_id: &UuidV4,
  original_enabler: &RuleEnabler,
  modified_enabler: &RuleEnabler,
) -> Result<(), SetRuleEnablerError> {
  let x = match location {
    Location::UserDeviceAccessRegulation { .. } => {
      table::set_rule_enabler(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        rule_id, 
        original_enabler, 
        modified_enabler,
      ).await
    }
    Location::UserAccountAccessRegulation { .. } => {
      table::set_rule_enabler(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        rule_id, 
        original_enabler, 
        modified_enabler,
      ).await
    }
    Location::UserInternetAccessRegulation { .. } => {
      table::set_rule_enabler(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        rule_id, 
        original_enabler, 
        modified_enabler,
      ).await
    }
  };

  if let Err(error) = x {
    eprintln!("{error:?}");
    return Err(SetRuleEnablerError::Other);
  }

  Ok(())
}

pub enum TableSpecifier {
  UserDeviceAccessRegulation,
  UserAccountAccessRegulation,
  UserInternetAccessRegulation,
}

pub async fn select_all_rules<T>(
  database: &Database,
  table_specifier: TableSpecifier,
  for_each: T
) -> Result<(), TextualError>
where 
  T: Fn(table::Row)
{
  match table_specifier {
    TableSpecifier::UserAccountAccessRegulation => {
      table::select_all_rules(
        &database.connection, 
        &database.user_account_access_regulation_rule_table, 
        for_each,
      ).await
    }
    TableSpecifier::UserDeviceAccessRegulation => {
      table::select_all_rules(
        &database.connection, 
        &database.user_device_access_regulation_rule_table, 
        for_each,
      ).await
    }
    TableSpecifier::UserInternetAccessRegulation => {
      table::select_all_rules(
        &database.connection, 
        &database.user_internet_access_regulation_rule_table, 
        for_each,
      ).await
    }
  }
}
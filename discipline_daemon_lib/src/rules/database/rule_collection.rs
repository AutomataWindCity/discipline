use crate::{rules::{CachedRuleActivator, CachedRuleEnabler}, x::UuidV4};
use crate::x::database::*;
use super::{RuleOwnerLocator, user_rule_collection, Connection};

pub struct RuleColections {
  user_device_access_regulation: user_rule_collection::Collection,
  user_account_access_regulation: user_rule_collection::Collection,
  user_internet_access_regulation: user_rule_collection::Collection,
}

pub struct RuleCollectionsProcedures<'a> {
  connection: &'a Connection,
  collections: &'a RuleColections,
}

impl<'a> RuleCollectionsProcedures<'a> {
  pub async fn add_rule(
    &self, 
    rule_id: &UuidV4, 
    rule_activator: &CachedRuleActivator, 
    rule_enabler: &CachedRuleEnabler, 
    rule_owner_locator: &RuleOwnerLocator,
  ) -> Result<(), ExecuteError> {
    match rule_owner_locator {
      RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
        user_rule_collection::add_rule(
          self.connection, 
          &self.collections.user_account_access_regulation, 
          rule_activator, 
          rule_enabler, 
          rule_id, 
          user_id,
        ).await
      }
      RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
        user_rule_collection::add_rule(
          self.connection, 
          &self.collections.user_device_access_regulation, 
          rule_activator, 
          rule_enabler, 
          rule_id, 
          user_id,
        ).await
      }
      RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
        user_rule_collection::add_rule(
          self.connection, 
          &self.collections.user_internet_access_regulation, 
          rule_activator, 
          rule_enabler, 
          rule_id, 
          user_id,
        ).await
      }
    }
  }

  pub fn remove_rule(&self, rule_id: &UuidV4, rule_owner_locator: &RuleOwnerLocator) -> Result<(), ExecuteError> {
    todo!()
  }
}

pub enum AddRuleError {
  DuplicateId,
  NoSuchOwner,
  Other,
}

pub async fn add_rule(
  database: &crate::daemon::Database,
  rule_id: &UuidV4, 
  rule_activator: &CachedRuleActivator, 
  rule_enabler: &CachedRuleEnabler, 
  rule_owner_locator: &RuleOwnerLocator,
) -> Result<(), AddRuleError> {
  match rule_owner_locator {
    RuleOwnerLocator::UserAccountAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_account_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
    RuleOwnerLocator::UserDeviceAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_device_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
    RuleOwnerLocator::UserInternetAccessRegulation { user_id } => {
      user_rule_collection::add_rule(
        &database.connection, 
        &database.user_internet_access_regulation_rule_collection, 
        rule_activator, 
        rule_enabler, 
        rule_id, 
        user_id,
      ).await
    }
  }
}

pub enum RemoveRuleError {
  NoSuchRule,
  Other,
}

pub async fn remove_rule(
  database: &crate::daemon::Database,
  rule_id: &UuidV4, 
  rule_owner_locator: &RuleOwnerLocator,
) -> Result<(), RemoveRuleError> {
  todo!()
}
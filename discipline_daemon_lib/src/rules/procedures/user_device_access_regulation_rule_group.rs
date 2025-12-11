use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, Daemon};
use crate::x::rules::*;
use super::core;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRule {
  user_id: UuidV4,
  rule_id: Option<UuidV4>,
  rule_activator_creator: RuleActivatorCreator,
  rule_enabler_creator: RuleEnablerCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddRuleReturn {
  NoSuchUser,
  TooManyRules,
  DuplicateId,
  InternalError,
  Success,
}

impl AddRule {
  pub async fn execute(self, daemon: &Daemon) -> AddRuleReturn {
    let users = daemon.state.users.read().await;

    let Some(user) = users.get_user(&self.user_id) else {
      return AddRuleReturn::NoSuchUser;
    };
    
    let user = &mut *user.write().await;
    let rule_group = &mut user.regulation_info.block_device_access.rules;
    let cross_group_info = &mut *daemon.state.rules.write().await;

    let return_value = core::add_rule(
      &daemon.database, 
      &Location::UserDeviceAccessRegulation { user_id: self.user_id },
      rule_group, 
      cross_group_info, 
      self.rule_id, 
      self.rule_activator_creator, 
      self.rule_enabler_creator,
    ).await;
    
    match return_value {
      core::AddRuleReturn::DuplicateId => AddRuleReturn::DuplicateId,
      core::AddRuleReturn::TooManyRules => AddRuleReturn::TooManyRules,
      core::AddRuleReturn::InternalError => AddRuleReturn::InternalError,
      core::AddRuleReturn::Success => AddRuleReturn::Success,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRule {
  user_id: UuidV4,
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  NoSuchUser,
  NoSuchRule,
  RuleStillEnabled,
  InternalError,
  Success,
}

impl DeleteRule {
  pub async fn execute(self, daemon: &Daemon) -> DeleteRuleReturn {
    let users = daemon.state.users.read().await;

    let Some(user) = users.get_user(&self.user_id) else {
      return DeleteRuleReturn::NoSuchUser;
    };
    
    let now = daemon.state.clock.read().await.now();
    let user = &mut *user.write().await;
    let rule_group = &mut user.regulation_info.block_device_access.rules;
    let cross_group_info = &mut *daemon.state.rules.write().await;

    let return_value = core::delete_rule(
      &daemon.database, 
      &Location::UserDeviceAccessRegulation { user_id: self.user_id },
      rule_group, 
      cross_group_info, 
      &self.rule_id, 
      now,
    ).await;
    
    match return_value {
      core::DeleteRuleReturn::InternalError => DeleteRuleReturn::InternalError,
      core::DeleteRuleReturn::NoSuchRule => DeleteRuleReturn::NoSuchRule,
      core::DeleteRuleReturn::RuleStillEnabled => DeleteRuleReturn::RuleStillEnabled,
      core::DeleteRuleReturn::Success => DeleteRuleReturn::Success,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateRule {
  user_id: UuidV4,
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivateRuleReturn {
  NoSuchUser,
  NoSuchRule,
  InternalError,
  Success,
}

impl ActivateRule {
  pub async fn execute(self, daemon: &Daemon) -> ActivateRuleReturn {
    let users = daemon.state.users.read().await;

    let Some(user) = users.get_user(&self.user_id) else {
      return ActivateRuleReturn::NoSuchUser;
    };
    
    let now = daemon.state.clock.read().await.now();
    let user = &mut *user.write().await;
    let rule_group = &mut user.regulation_info.block_device_access.rules;

    let return_value = core::activate_rule(
      &daemon.database, 
      &Location::UserDeviceAccessRegulation { user_id: self.user_id }, 
      rule_group, 
      &self.rule_id, 
      now,
    ).await;
    
    match return_value {
      core::ActivateRuleReturn::Success => ActivateRuleReturn::Success,
      core::ActivateRuleReturn::NoSuchRule => ActivateRuleReturn::NoSuchRule,
      core::ActivateRuleReturn::InternalError => ActivateRuleReturn::InternalError,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivateRule {
  user_id: UuidV4,
  rule_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeactivateRuleReturn {
  NoSuchUser,
  NoSuchRule,
  InternalError,
  Success,
}

impl DeactivateRule {
  pub async fn execute(self, daemon: &Daemon) -> DeactivateRuleReturn {
    let users = daemon.state.users.read().await;

    let Some(user) = users.get_user(&self.user_id) else {
      return DeactivateRuleReturn::NoSuchUser;
    };
    
    let now = daemon.state.clock.read().await.now();
    let user = &mut *user.write().await;
    let rule_group = &mut user.regulation_info.block_device_access.rules;

    let return_value = core::deactivate_rule(
      &daemon.database, 
      &Location::UserDeviceAccessRegulation { user_id: self.user_id }, 
      rule_group, 
      &self.rule_id, 
      now,
    ).await;
    
    match return_value {
      core::DeactivateRuleReturn::InternalError => DeactivateRuleReturn::InternalError,
      core::DeactivateRuleReturn::NoSuchRule => DeactivateRuleReturn::NoSuchRule,
      core::DeactivateRuleReturn::Success => DeactivateRuleReturn::Success,
    }
  }
}

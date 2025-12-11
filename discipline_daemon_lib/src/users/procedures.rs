use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::x::{MonotonicInstant, User, UserGroup, UserName, UuidV4, Database, operating_system, Daemon};
use super::database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddUserReturn {
  TooManyUsers,
  DuplicateId,
  NoSuchOperatingSystemUser,
  InternalError,
  Success,
}

pub async fn add_user(
  database: &Database,
  user_group: &mut UserGroup,
  user_id: Option<UuidV4>,
  user_name: UserName,
  operating_system_user_name: operating_system::UserName,
) -> AddUserReturn {
  if user_group.users.len() >= user_group.maximum_user_number {
    return AddUserReturn::TooManyUsers;
  }
  
  let operating_system_info = match operating_system::get_per_user_info(&operating_system_user_name) {
    Ok(info) => {
      info
    }
    Err(operating_system::GetPerUserInfoError::NoSuchUser) => {
      return AddUserReturn::NoSuchOperatingSystemUser;
    }
    Err(_) => {
      return AddUserReturn::InternalError;
    }
  };

  let user_id_is_created_by_client = user_id.is_some();
  let user_id = user_id.unwrap_or_else(UuidV4::generate);
  let user = User::new(operating_system_info);

  if let Err(error) = database::add_user(
    database, 
    &user_id,
    &user_name,
    &user.regulation_info,
    &user.operating_system_info,
  ).await {
    return match error {
      database::AddUserError::DuplicateId => {
        if user_id_is_created_by_client {
          AddUserReturn::DuplicateId
        } else {
          AddUserReturn::InternalError
        }
      }
      database::AddUserError::Other => {
        AddUserReturn::InternalError
      }
    };
  }

  user_group.users.insert(user_id, Arc::new(RwLock::new(user)));

  AddUserReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteUserReturn {
  NoSuchUser,
  SomeRulesStillEnabled,
  InternalError,
  Success,
}

pub async fn delete_user(
  database: &Database,
  user_group: &mut UserGroup,
  user_id: &UuidV4,
  now: MonotonicInstant,
) -> DeleteUserReturn {
  let Some(user) = user_group.users.get(user_id) else {
    return DeleteUserReturn::NoSuchUser;
  };

  let user = user.read().await;
  if user.regulation_info.has_enabled_rules(now) {
    return DeleteUserReturn::SomeRulesStillEnabled;
  }

  if let Err(error) = database::delete_user(
    database, 
    user_id,
  ).await {
    return match error {
      database::DeleteUserError::Other => {
        DeleteUserReturn::InternalError
      }
      database::DeleteUserError::NoSuchUser => {
        DeleteUserReturn::InternalError
      }
    }
  }

  drop(user);
  user_group.users.remove(user_id);

  DeleteUserReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetUserNameReturn {
  NoSuchUser,
  InternalError,
  Success,
}

pub async fn set_user_name(
  database: &Database,
  user_group: &UserGroup,
  user_id: &UuidV4,
  new_user_name: &UserName,
) -> SetUserNameReturn {
  if !user_group.users.contains_key(user_id) {
    return SetUserNameReturn::NoSuchUser;
  }

  if let Err(error) = database::change_user_name(
    database, 
    user_id, 
    &new_user_name,
  ).await {
    return match error {
      database::ChangeUserNameError::Other => {
        SetUserNameReturn::InternalError
      }
      database::ChangeUserNameError::NoSuchUser => {
        SetUserNameReturn::InternalError
      }
    };
  }

  SetUserNameReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUser {
  user_id: Option<UuidV4>,
  user_name: UserName,
  operating_system_user_name: operating_system::UserName,
}

impl AddUser {
  pub async fn execute(self, daemon: &Daemon) -> AddUserReturn {
    let mut user_group_guard = daemon.state.users.write().await;
    let user_group = &mut *user_group_guard;

    add_user(
      &daemon.database, 
      user_group, 
      self.user_id,
      self.user_name, 
      self.operating_system_user_name,
    ).await
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUser {
  user_id: UuidV4,
}

impl DeleteUser {
  pub async fn execute(self, daemon: &Daemon) -> DeleteUserReturn {
    let mut user_group_guard = daemon.state.users.write().await;
    let user_group = &mut *user_group_guard;
    let now = daemon.state.clock.read().await.now();

    delete_user(
      &daemon.database, 
      user_group, 
      &self.user_id, 
      now,
    ).await
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserName {
  user_id: UuidV4,
  new_user_name: UserName,
}

impl SetUserName {
  pub async fn execute(self, daemon: &Daemon) -> SetUserNameReturn {
    let user_group_guard = daemon.state.users.read().await;
    let user_group = &*user_group_guard;

    set_user_name(
      &daemon.database, 
      user_group, 
      &self.user_id, 
      &self.new_user_name,
    ).await
  }
}
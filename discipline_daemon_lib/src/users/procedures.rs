use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::x::{MonotonicInstant, User, UserGroup, UserName, UuidV4, Database, operating_system};

mod database {
  use crate::x::{Database, User, UserName, UuidV4};

  pub enum AddUserError {
    DuplicateId,
    InternalError,
  }

  pub async fn add_user(
    database: &Database,
    user_id: &UuidV4,
    user_name: &UserName,
    per_user_regulation_info: &crate::x::regulation::PerUserInfo,
    per_user_operating_system_info: &crate::x::operating_system::PerUserInfo,
  ) -> Result<(), AddUserError> {
    todo!()
  }

  pub enum DeleteUserError {
    NoSuchUser,
    InternalError,
  }

  pub async fn delete_user(
    database: &Database,
    user_id: &UuidV4,
  ) -> Result<(), DeleteUserError> {
    todo!()
  }

  pub enum ChangeUserNameError {
    NoSuchUser,
    InternalError,
  }

  pub async fn change_user_name(
    database: &Database,
    user_id: &UuidV4,
    new_user_name: &UserName
  ) -> Result<(), ChangeUserNameError> {
    todo!()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUser {
  user_id: Option<UuidV4>,
  user_name: UserName,
  operating_system_user_name: operating_system::UserName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddUserReturn {
  TooManyUsers,
  DuplicateId,
  NoSuchOperatingSystemUser,
  InternalError,
  Success,
}

impl AddUser {
  pub async fn execute(
    self,
    database: &Database,
    user_group: &mut UserGroup,
  ) -> AddUserReturn {
    if user_group.users.len() >= user_group.maximum_user_number {
      return AddUserReturn::TooManyUsers;
    }
    
    let operating_system_info = match operating_system::get_per_user_info(&self.operating_system_user_name) {
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

    let user_id_is_created_by_client = self.user_id.is_some();
    let user_id = self.user_id.unwrap_or_else(UuidV4::generate);
    let user = User::new(operating_system_info);

    if let Err(error) = database::add_user(
      database, 
      &user_id,
      &self.user_name,
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
        database::AddUserError::InternalError => {
          AddUserReturn::InternalError
        }
      };
    }

    user_group.users.insert(user_id, Arc::new(Mutex::new(user)));

    AddUserReturn::Success
  }
}

pub struct DeleteUser {
  user_id: UuidV4,
}

pub enum DeleteUserReturn {
  NoSuchUser,
  SomeRulesStillEnabled,
  InternalError,
  Success,
}

impl DeleteUser {
  pub async fn execute(
    self,
    now: MonotonicInstant,
    database: &Database,
    user_group: &mut UserGroup,
  ) -> DeleteUserReturn {
    let Some(user) = user_group.users.get(&self.user_id) else {
      return DeleteUserReturn::NoSuchUser;
    };

    let user = user.lock().await;
    if user.regulation_info.has_enabled_rules(now) {
      return DeleteUserReturn::SomeRulesStillEnabled;
    }

    if let Err(error) = database::delete_user(
      database, 
      &self.user_id,
    ).await {
      return match error {
        database::DeleteUserError::InternalError => {
          DeleteUserReturn::InternalError
        }
        database::DeleteUserError::NoSuchUser => {
          DeleteUserReturn::InternalError
        }
      }
    }

    drop(user);
    user_group.users.remove(&self.user_id);

    DeleteUserReturn::Success
  }
}

pub struct UpdateName {
  user_id: UuidV4,
  new_user_name: UserName
}

pub enum UpdateNameReturn {
  NoSuchUser,
  InternalError,
  Success,
}

impl UpdateName {
  pub async fn execute(
    self,
    database: &Database,
    user_group: &mut UserGroup,
  ) -> UpdateNameReturn {
    if !user_group.users.contains_key(&self.user_id) {
      return UpdateNameReturn::NoSuchUser;
    }

    if let Err(error) = database::change_user_name(
      database, 
      &self.user_id, 
      &self.new_user_name,
    ).await {
      return match error {
        database::ChangeUserNameError::InternalError => {
          UpdateNameReturn::InternalError
        }
        database::ChangeUserNameError::NoSuchUser => {
          UpdateNameReturn::InternalError
        }
      };
    }

    UpdateNameReturn::Success
  }
}
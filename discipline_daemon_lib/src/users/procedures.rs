use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::x::{MonotonicInstant, User, UserGroup, UserName, UuidV4, Database, operating_system};
use super::database;

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
        database::AddUserError::Other => {
          AddUserReturn::InternalError
        }
      };
    }

    user_group.users.insert(user_id, Arc::new(Mutex::new(user)));

    AddUserReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUser {
  user_id: UuidV4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        database::DeleteUserError::Other => {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserName {
  user_id: UuidV4,
  new_user_name: UserName
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetUserNameReturn {
  NoSuchUser,
  InternalError,
  Success,
}

impl SetUserName {
  pub async fn execute(
    self,
    database: &Database,
    user_group: &mut UserGroup,
  ) -> SetUserNameReturn {
    if !user_group.users.contains_key(&self.user_id) {
      return SetUserNameReturn::NoSuchUser;
    }

    if let Err(error) = database::change_user_name(
      database, 
      &self.user_id, 
      &self.new_user_name,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Procedure {
  AddUser(AddUser),
  DeleteUser(DeleteUser),
  SetUserName(SetUserName)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcedureReturn {
  AddUser(AddUserReturn),
  DeleteUser(DeleteUserReturn),
  SetUserName(SetUserNameReturn),
}

impl Procedure {
  pub async fn execute(
    self,
    now: MonotonicInstant,
    database: &Database,
    user_group: &mut UserGroup,
  ) -> ProcedureReturn {
    match self {
      Self::AddUser(inner) => {
        ProcedureReturn::AddUser(
          inner
            .execute(database, user_group)
            .await
        )
      }
      Self::DeleteUser(inner) => {
        ProcedureReturn::DeleteUser(
          inner
            .execute(now, database, user_group)
            .await
        )
      }
      Self::SetUserName(inner) => {
        ProcedureReturn::SetUserName(
          inner
            .execute(database, user_group)
            .await
        )
      }
    }
  }
}
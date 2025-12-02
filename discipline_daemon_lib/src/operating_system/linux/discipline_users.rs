use serde::{Deserialize, Serialize};
use super::{UserId, UserName, AllocationConfig, GetPasswordFileEntryError, get_password_file_entry_with_user_name};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerUserInfo {
  pub(super) user_id: UserId,
  pub(super) user_name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrossUserInfo {

}

pub type GetPerUserInfoError = GetPasswordFileEntryError;

const CONFIG: AllocationConfig = AllocationConfig {
  maximum_memory_allocation_retries: 3,
  memory_allocation_increment_factor: 3,
};

pub fn get_per_user_info(user_name: &UserName) -> Result<PerUserInfo, GetPerUserInfoError> {
  get_password_file_entry_with_user_name(user_name, &CONFIG)
    .map(|it| PerUserInfo { 
      user_id: it.user_id, 
      user_name: it.user_name,
    })
}

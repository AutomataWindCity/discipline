mod x;
mod n;
use std::ffi::CString;

pub use n::{UserId, UserName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerUserInfo {
  user_id: UserId,
  user_name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrossUserInfo {

}

pub fn get_user_name_given_user_() {

}


pub type GetPerUserInfoError = n::GetPasswordFileEntryError;

const CONFIG: n::AllocationConfig = n::AllocationConfig {
  maximum_memory_allocation_retries: 3,
  memory_allocation_increment_factor: 3,
};

pub fn get_per_user_info(user_name: &UserName) -> Result<PerUserInfo, GetPerUserInfoError> {
  n::get_password_file_entry_with_user_name(user_name, &CONFIG)
    .map(|it| PerUserInfo { 
      user_id: it.user_id, 
      user_name: it.user_name,
    })
}

mod serialization {
  use std::ffi::CString;

use serde::{Serialize, Deserialize, de::Error};
  use crate::x::TextualError;

use super::{UserId, UserName};

  impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.inner().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u32::deserialize(deserializer)
        .map(UserId::new)
        .map_err(|error| {
          Error::custom(
            TextualError::new("Reading UserId")
              .with_message("UserId is a linux user id represented as u32")
              .with_message("Failed to read a u32 value")
              .with_attachement_display("Error", error)
          )
        })
    }
  }

  impl Serialize for UserName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.inner().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for UserName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      CString::deserialize(deserializer)
        .map(UserName::new)
        .map_err(|error| {
          Error::custom(
            TextualError::new("Reading UserName")
              .with_message("UserName is a linux user name represented as String")
              .with_message("Failed to read a CString value")
              .with_attachement_display("Error", error)
          )
        })
    }
  }
}
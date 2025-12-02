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
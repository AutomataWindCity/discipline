use serde::{Serialize, Deserialize, de::Error};
use crate::x::{Time, ToTextualError};

impl Serialize for Time {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer 
  {
    self.as_timestamp().serialize(serializer)
  }
}

impl<'a> Deserialize<'a> for Time {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a> 
  {
    let timestamp = u32::deserialize(deserializer)?;
      
    Time::from_timestamp(timestamp)
      .map_err(|error| {
        Error::custom(error.to_textual_error())
      })
  }
}
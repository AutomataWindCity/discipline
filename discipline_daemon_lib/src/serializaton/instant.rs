use serde::{Serialize, Deserialize};
use crate::x::Instant;

impl Serialize for Instant {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer 
  {
    self.as_elapsed_time().serialize(serializer)
  }
}

impl<'a> Deserialize<'a> for Instant {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a> 
  {
    Duration::deserialize(deserializer).map(|elapsed_time| Instant::from_elapsed_time(elapsed_time))
  }
}
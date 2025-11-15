use crate::x::Duration;

pub struct MonotonicClock {
  milliseconds: u64,
}

impl MonotonicClock {
  pub fn now(&self) -> MonotonicInstant {
    MonotonicInstant { timestamp: self.milliseconds }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotonicInstant {
  timestamp: u64,
}

impl MonotonicInstant {
  pub const MAX: MonotonicInstant = MonotonicInstant { timestamp: u64::MAX };

  pub fn from_timestamp(timestamp: u64) -> Self {
    Self { timestamp }
  }
  
  pub fn is_eariler_than(self, other: MonotonicInstant) -> bool {
    self.timestamp < other.timestamp
  }

  pub fn is_later_than(self, other: MonotonicInstant) -> bool {
    self.timestamp > other.timestamp
  }

  pub fn is_at(self, other: MonotonicInstant) -> bool {
    self.timestamp == other.timestamp
  }

  pub fn till_or_zero(self, other: MonotonicInstant) -> Duration {
    other
      .timestamp
      .checked_sub(self.timestamp)
      .map(Duration::from_milliseconds)
      .unwrap_or_else(Duration::zero)
  }

  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }
}

mod serialization {
  use serde::{Serialize, Deserialize};
  use super::{MonotonicInstant};

  impl Serialize for MonotonicInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.timestamp.serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for MonotonicInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u64::deserialize(deserializer).map(|timestamp| MonotonicInstant { timestamp })
    }
  }
}
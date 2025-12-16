#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
  milliseconds: u64,
}

impl Duration {
  pub fn from_milliseconds(milliseconds: u64) -> Duration {
    Self { milliseconds }
  }

  pub const fn from_minutes_or_panic(minutes: u64) -> Duration {
    let Some(milliseconds) = minutes.checked_mul(1000 * 60) else {
      // TODO: Write a proper error message.
      panic!("Creating ");
    };


    Self { milliseconds }
  }

  pub fn zero() -> Duration {
    Self { milliseconds: 0 }
  }

  pub fn is_zero(self) -> bool {
    self.milliseconds == 0
  }

  pub fn minus_or_zero(self, rhs: Self) -> Duration {
    match self
      .milliseconds
      .checked_sub(rhs.milliseconds)
    {
      None => {
        Duration::zero()
      }
      Some(milliseconds) => {
        Duration { milliseconds }
      }
    }
  }

  pub fn milliseconds(self) -> u64 {
    self.milliseconds
  }

  pub fn to_std_duration(self) -> std::time::Duration {
    std::time::Duration::from_millis(self.milliseconds)
  }
}

mod serialization {
  use serde::{Serialize, Deserialize};
  use crate::x::Duration;

  impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.milliseconds().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u64::deserialize(deserializer).map(Duration::from_milliseconds)
    }
  }
}
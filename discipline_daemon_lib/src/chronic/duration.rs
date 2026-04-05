#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
  milliseconds: u64,
}

impl Duration {
  pub const MILLISECONDS_PER_SECOND: u64 = 1000;
  pub const MILLISECONDS_PER_MINUTE: u64 = 1000 * 60;
  pub const MILLISECONDS_PER_HOUR: u64 = 1000 * 60 * 60;
  pub const MILLISECONDS_PER_DAY: u64 = 1000 * 60 * 60 * 24;
  pub const MILLISECONDS_PER_WEEK: u64 = 1000 * 60 * 60 * 24 * 7;

  pub const MAXIMUM_SECONDS: u64 = u64::MAX / Self::MILLISECONDS_PER_SECOND;
  pub const MAXIMUM_MINUTES: u64 = u64::MAX / Self::MILLISECONDS_PER_MINUTE;
  pub const MAXIMUM_HOURS: u64 = u64::MAX / Self::MILLISECONDS_PER_HOUR;
  pub const MAXIMUM_DAYS: u64 = u64::MAX / Self::MILLISECONDS_PER_DAY;
  pub const MAXIMUM_WEEKS: u64 = u64::MAX / Self::MILLISECONDS_PER_WEEK;

  pub const MIN: Duration = Duration::from_milliseconds(0);
  pub const MAX: Duration = Duration::from_milliseconds(u64::MAX);
  
  pub const SECOND: Duration = Duration::from_milliseconds(Self::MILLISECONDS_PER_SECOND);
  pub const MINUTE: Duration = Duration::from_milliseconds(Self::MILLISECONDS_PER_MINUTE);
  pub const HOUR: Duration = Duration::from_milliseconds(Self::MILLISECONDS_PER_HOUR);
  pub const DAY: Duration = Duration::from_milliseconds(Self::MILLISECONDS_PER_DAY);
  pub const WEEK: Duration = Duration::from_milliseconds(Self::MILLISECONDS_PER_WEEK);

  pub const fn from_milliseconds(milliseconds: u64) -> Duration {
    Self { milliseconds }
  }

  pub const fn from_minutes_or_panic(minutes: u64) -> Duration {
    let Some(milliseconds) = minutes.checked_mul(1000 * 60) else {
      // TODO: Write a proper error message.
      panic!("Creating ");
    };


    Self { milliseconds }
  }

  pub const fn zero() -> Duration {
    Self { milliseconds: 0 }
  }

  pub const fn day() -> Duration {
    Duration { milliseconds: 1000 * 60 * 60 * 24 }
  }

  pub const fn week() -> Duration {
    Duration { milliseconds: 1000 * 60 * 60 * 24 * 7 }
  }

  pub fn is_zero(self) -> bool {
    self.milliseconds == 0
  }

  pub fn is_shorter_than(self, other: Duration) -> bool {
    self < other
  }
  pub fn is_shorter_than_or_equal_to(self, other: Duration) -> bool {
    self <= other
  }
  pub fn is_equal_to(self, other: Duration) -> bool {
    self == other
  }
  pub fn is_longer_than(self, other: Duration) -> bool {
    self > other
  }
  pub fn is_longer_than_or_equal_to(self, other: Duration) -> bool {
    self >= other
  }
  
  pub fn saturating_sub(self, rhs: Self) -> Duration {
    Duration::from_milliseconds(self.milliseconds.saturating_sub(rhs.milliseconds))
  }

  pub fn saturating_add(self, rhs: Self) -> Duration {
    Duration::from_milliseconds(self.milliseconds.saturating_add(rhs.milliseconds))
  }

  pub fn div_or_zero(self, rhs: Duration) -> Duration {
    match self.milliseconds.checked_div(rhs.milliseconds) {
      Some(result) => {
        Duration::from_milliseconds(result)
      }
      None => {
        Duration::zero()
      }
    }
  }

  pub fn rem_or_zero(self, rhs: Duration) -> Duration {
    match self.milliseconds.checked_rem(rhs.milliseconds) {
      Some(result) => {
        Duration::from_milliseconds(result)
      }
      None => {
        Duration::zero()
      }
    }
  }

  pub fn as_total_milliseconds(self) -> u64 {
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
      self.as_total_milliseconds().serialize(serializer)
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
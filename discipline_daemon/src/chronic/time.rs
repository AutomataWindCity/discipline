const MINIMUM_TIMESTAMP: u32 = 0;
const MAXIMUM_TIMESTAMP: u32 = 1000 * 60 * 60 * 24 - 1;

#[derive(Debug, Clone)]
pub enum CreateFromTimestampError {
  MaximumLengthViolation { timestamp: u32 },
  MinimumLengthViolation { timestamp: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
  timestamp: u32
}

impl Time {
  pub fn from_millisecond_timestamp(timestamp: u32) -> Result<Time, CreateFromTimestampError> {
    if timestamp < MINIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::MinimumLengthViolation { timestamp });
    }
    if timestamp > MAXIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::MaximumLengthViolation { timestamp });
    }
    Ok(Time { timestamp })
  }

  pub fn millisecond_timestamp(&self) -> u32 {
    self.timestamp
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::{Time, time};

  impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.millisecond_timestamp().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      let timestamp = u32::deserialize(deserializer)?;
      Time::from_millisecond_timestamp(timestamp).map_err(|error| match error {
        time::CreateFromTimestampError::MaximumLengthViolation { timestamp } => {
          Error::custom(format!("Deserializing Time: Creating Time from timestamp: Provided timestamp is larger than the maximum value. Timestamp is {timestamp}. Maximum value is {}.", time::MAXIMUM_TIMESTAMP))
        }
        time::CreateFromTimestampError::MinimumLengthViolation { timestamp } => {
          Error::custom(format!("Deserializing Time: Creating Time from timestamp: Provided timestamp is less than the minimum value. Timestamp is {timestamp}. Minimum value is {}.", time::MINIMUM_TIMESTAMP))
        }
      })
    }
  }
}
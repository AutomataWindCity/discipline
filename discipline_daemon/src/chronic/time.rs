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

  pub fn timestamp(&self) -> u32 {
    self.timestamp
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::Time;

  impl Serialize for Time {
    
  }
}
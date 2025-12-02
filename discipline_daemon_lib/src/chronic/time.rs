use crate::x::{TextualErrorContext, ToTextualError};

pub const MINIMUM_TIMESTAMP: u32 = 0;
pub const MAXIMUM_TIMESTAMP: u32 = 1000 * 60 * 60 * 24 - 1;

#[derive(Debug, Clone)]
pub enum CreateFromTimestampError {
  TimestampOutOfRange { timestamp: u32 },
}

impl ToTextualError for CreateFromTimestampError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new("Creating Time from a millisecond-based timestamp since midnight");
    
    match self {
      Self::TimestampOutOfRange { timestamp } => {
        context.add_message("Timestamp is outside the valid range");
        context.add_attachement_display("Timestamp", timestamp);
        context.add_attachement_display("Minimum valid value", MINIMUM_TIMESTAMP);
        context.add_attachement_display("Maximum valid value", MAXIMUM_TIMESTAMP);
      }
    }

    context
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
  timestamp: u32
}

impl Time {
  pub unsafe fn unchecked_from_millisecond_timestamp(timestamp: u32) -> Time {
    Time { timestamp }
  }

  pub fn from_millisecond_timestamp(timestamp: u32) -> Result<Time, CreateFromTimestampError> {
    if timestamp < MINIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::TimestampOutOfRange { timestamp });
    }
    if timestamp > MAXIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::TimestampOutOfRange { timestamp });
    }
    Ok(Time { timestamp })
  }

  pub fn millisecond_timestamp(&self) -> u32 {
    self.timestamp
  }

  pub fn five_minute_timestamp_usize(&self) -> usize {
    (self.timestamp as usize) / (1000 * 60 * 5)
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::{Time, ToTextualError};

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
        
      Time::from_millisecond_timestamp(timestamp)
        .map_err(|error| {
          Error::custom(error.to_textual_error())
        })
    }
  }
}
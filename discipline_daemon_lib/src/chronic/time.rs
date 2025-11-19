use crate::x::{TextualErrorContext, ToTextualError};

const MINIMUM_TIMESTAMP: u32 = 0;
const MAXIMUM_TIMESTAMP: u32 = 1000 * 60 * 60 * 24 - 1;

#[derive(Debug, Clone)]
pub enum CreateFromTimestampError {
  MaximumLengthViolation { timestamp: u32 },
  MinimumLengthViolation { timestamp: u32 },
}

impl ToTextualError for CreateFromTimestampError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new("Creating Time from a millisecond-based timestamp since midnight");
    
    match self {
      Self::MinimumLengthViolation { timestamp } => {
        context.add_message("Timestamp is less than the minimum valid value");
        context.add_attachement_display("Timestamp", timestamp);
        context.add_attachement_display("Minimum valid value", MINIMUM_TIMESTAMP);
        context.add_attachement_display("Maximum valid value", MAXIMUM_TIMESTAMP);
      }
      Self::MaximumLengthViolation { timestamp } => {
        context.add_message("Timestamp is greater than the maximum valid value");
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
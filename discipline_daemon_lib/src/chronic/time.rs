use crate::x::{Duration, TextualErrorContext, ToTextualError};

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
        context.add_attachement_display("Minimum valid value", Time::MINIMUM_TIMESTAMP);
        context.add_attachement_display("Maximum valid value", Time::MAXIMUM_TIMESTAMP);
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
  pub const MINIMUM_TIMESTAMP: u32 = 0;
  pub const MAXIMUM_TIMESTAMP: u32 = 1000 * 60 * 60 * 24 - 1;

  pub unsafe fn unchecked_from_timestamp(timestamp: u32) -> Time {
    Time { timestamp }
  }

  pub fn from_timestamp(timestamp: u32) -> Result<Time, CreateFromTimestampError> {
    if timestamp < Time::MINIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::TimestampOutOfRange { timestamp });
    }
    if timestamp > Time::MAXIMUM_TIMESTAMP {
      return Err(CreateFromTimestampError::TimestampOutOfRange { timestamp });
    }
    Ok(Time { timestamp })
  }

  pub fn as_timestamp(&self) -> u32 {
    self.timestamp
  }

  pub fn as_elapsed_time(&self) -> Duration {
    Duration::from_milliseconds(self.timestamp as u64)
  }
}
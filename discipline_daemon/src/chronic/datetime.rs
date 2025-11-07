use crate::x::Duration;

#[derive(Debug, Clone)]
pub enum CreateFromMillisecondTimestampError {
  RangeViolation { timestamp: i64 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
  inner: chrono::DateTime<chrono::Utc>,
}

impl DateTime {
  pub fn now() -> DateTime {
    Self {
      inner: chrono::Utc::now(),
    }
  }

  pub fn from_millisecond_timestamp(timestamp: i64) -> Result<DateTime, CreateFromMillisecondTimestampError> {
    match chrono::DateTime::from_timestamp_millis(timestamp) {
      Some(inner) => {
        Ok(DateTime { inner })
      }
      None => {
        Err(CreateFromMillisecondTimestampError::RangeViolation { timestamp })
      }
    }
  }

  pub fn millisecond_timestamp(self) -> i64 {
    self.inner.timestamp_millis()
  }

  pub fn till_or_zero(self, later: DateTime) -> Duration {
    match later
      .millisecond_timestamp()
      .checked_sub(self.millisecond_timestamp())
    {
      None => {
        Duration::zero()
      }
      Some(milliseconds) => {
        Duration::from_milliseconds(milliseconds.try_into().unwrap())
      }
    }
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::{DateTime, TextualError, datetime};

  impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.millisecond_timestamp().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      let timestamp = i64::deserialize(deserializer).map_err(|error| {
        Error::custom(TextualError::new("Deserializing DateTime from i64 millisecond-based UTC timestamp")
          .with_message("Failed to deserialize value as an i64 number")
          .with_attachement_display("Error", error))
      })?;

      DateTime::from_millisecond_timestamp(timestamp).map_err(|error| match error {
        datetime::CreateFromMillisecondTimestampError::RangeViolation { timestamp } => {
          Error::custom(
            TextualError::new("Deserializing DateTime from i64 millisecond-based UTC timestamp")
              .with_message("Value is an i64 number, but it's outside the valid range for a millisecond-based UTC timestamp")
              .with_attachement_display("Value", timestamp)
          )
        }
      })
    }
  }
}
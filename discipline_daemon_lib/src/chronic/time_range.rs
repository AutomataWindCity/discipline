use serde::{Deserialize, Serialize};
use crate::x::{Duration, Time};

pub enum CreateFromTimestampsError {
  FromTimestampIsLessThanMinimumValue { from: u32, till: u32 },
  FromTimestampIsGreaterThanMaximumValue { from: u32, till: u32 },
  TillTimestampIsLessThanMinimumValue { from: u32, till: u32 },
  TillTimestampIsGreaterThanMaximumValue { from: u32, till: u32 },
  FromTimestampIsLaterThanTillTimestamp { from: u32, till: u32 },
  RangeIsEmpty { from: u32, till: u32 },
  RangeIsLongerThanOneDay { from: u32, till: u32 },
}

const MILLISECONDS_PER_DAY: u32 = 1000 * 60 * 60 * 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
  from: u32,
  till: u32,
}

impl TimeRange {
  pub const MINIMUM_FROM_VALUE: u32 = Time::MINIMUM_TIMESTAMP;
  pub const MAXIMUM_FROM_VALUE: u32 = Time::MAXIMUM_TIMESTAMP;

  pub const MINIMUM_TILL_VALUE: u32 = Time::MINIMUM_TIMESTAMP;
  pub const MAXIMUM_TILL_VALUE: u32 = Time::MAXIMUM_TIMESTAMP * 2 + 1;

  pub fn from_times(from: Time, till: Time) -> TimeRange {
    let from = from.as_timestamp();
    let till = till.as_timestamp();
    if from < till {
      TimeRange { from, till }
    } else {
      TimeRange { from, till: MILLISECONDS_PER_DAY + till }
    }
  }

  pub fn from(&self) -> Time {
    // TODO: Document this properly
    unsafe {
      Time::unchecked_from_timestamp(self.from)
    }
  }

  pub fn till(&self) -> Time {
    // TODO: Document this properly
    let timestamp = if self.till > Self::MAXIMUM_FROM_VALUE {
      self.till - MILLISECONDS_PER_DAY
    } else {
      self.till
    };

    // TODO: Document this properly
    unsafe {
      Time::unchecked_from_timestamp(timestamp)
    }
  }

  pub fn from_timestamps(from: u32, till: u32) -> Result<TimeRange, CreateFromTimestampsError> {
    if from < Self::MINIMUM_FROM_VALUE {
      return Err(CreateFromTimestampsError::FromTimestampIsLessThanMinimumValue { from, till });
    }
    if from > Self::MAXIMUM_FROM_VALUE {
      return Err(CreateFromTimestampsError::FromTimestampIsGreaterThanMaximumValue { from, till });
    }
    if till < Self::MINIMUM_TILL_VALUE {
      return Err(CreateFromTimestampsError::TillTimestampIsLessThanMinimumValue { from, till });
    }
    if till > Self::MAXIMUM_TILL_VALUE {
      return Err(CreateFromTimestampsError::TillTimestampIsGreaterThanMaximumValue { from, till });
    }
    if from > till {
      return Err(CreateFromTimestampsError::FromTimestampIsLaterThanTillTimestamp { from, till });
    }
    if from == till {
      return Err(CreateFromTimestampsError::RangeIsEmpty { from, till });
    }
    if till - from > MILLISECONDS_PER_DAY {
      return Err(CreateFromTimestampsError::RangeIsLongerThanOneDay { from, till });
    }
    Ok(TimeRange {
      from,
      till,
    })
  }

  pub fn contains(&self, time: Time) -> bool {
    let time = time.as_timestamp();
    self.from <= time && self.till >= time
  }

  pub fn duration(&self) -> Duration {
    Duration::from_milliseconds((self.till - self.from) as u64)
  }
}

use serde::{Deserialize, Serialize};
use crate::x::Time;

pub const MINIMUM_FROM_VALUE: u32 = 0;
pub const MAXIMUM_FROM_VALUE: u32 = 1000 * 60 * 60 * 24 - 1;

pub const MINIMUM_TILL_VALUE: u32 = 0;
pub const MAXIMUM_TILL_VALUE: u32 = 1000 * 60 * 60 * 24 * 2 - 1;

const MILLISECONDS_PER_DAY: u32 = 1000 * 60 * 60 * 24;

pub enum CreateFromTimestampsError {
  FromTimestampIsLessThanMinimumValue { from: u32, till: u32 },
  FromTimestampIsGreaterThanMaximumValue { from: u32, till: u32 },
  TillTimestampIsLessThanMinimumValue { from: u32, till: u32 },
  TillTimestampIsGreaterThanMaximumValue { from: u32, till: u32 },
  FromTimestampIsLaterThanTillTimestamp { from: u32, till: u32 },
  RangeIsEmpty { from: u32, till: u32 },
  RangeIsLongerThanOneDay { from: u32, till: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
  from: u32,
  till: u32,
}

impl TimeRange {
  pub fn from_times(from: Time, till: Time) -> TimeRange {
    let from = from.millisecond_timestamp();
    let till = till.millisecond_timestamp();
    if from < till {
      TimeRange {
        from,
        till,
      }
    } else {
      TimeRange {
        from,
        till: MILLISECONDS_PER_DAY + till,
      }
    }
  }

  pub fn from(&self) -> Time {
    todo!()
  }

  pub fn till(&self) -> Time {
    todo!()
  }

  pub fn from_timestamps(from: u32, till: u32) -> Result<TimeRange, CreateFromTimestampsError> {
    if from < MINIMUM_FROM_VALUE {
      return Err(CreateFromTimestampsError::FromTimestampIsLessThanMinimumValue { from, till });
    }
    if from > MAXIMUM_FROM_VALUE {
      return Err(CreateFromTimestampsError::FromTimestampIsGreaterThanMaximumValue { from, till });
    }
    if till < MINIMUM_TILL_VALUE {
      return Err(CreateFromTimestampsError::TillTimestampIsLessThanMinimumValue { from, till });
    }
    if till > MAXIMUM_TILL_VALUE {
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
    let time = time.millisecond_timestamp();
    self.from <= time && self.till >= time
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  from: Time,
  till: Time,
}

impl Creator {
  pub fn create(self) -> TimeRange {
    TimeRange::from_times(self.from, self.till)
  }
}
use serde::{Deserialize, Serialize};
use crate::x::{Time, TimeRange, Weekday, WeekdaySet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConditional {
  time_range: TimeRange,
  weekday_set: WeekdaySet,
}

impl TimeConditional {
  pub fn new(time_range: TimeRange, weekday_set: WeekdaySet) -> Self {
    Self {
      time_range,
      weekday_set,
    }
  }

  pub fn evaulate(&self, time: Time, weekday: Weekday) -> bool {
    self.time_range.contains(time)
    &&
    self.weekday_set.contains(weekday)
  }

  pub fn time_range(&self) -> TimeRange {
    self.time_range
  }

  pub fn weekday_set(&self) -> WeekdaySet {
    self.weekday_set
  }
}

// TODO: Implement Serialize manually and validate that time_range is ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  time_range: TimeRange,
  weekday_set: WeekdaySet,
}

impl Creator {
  pub fn create(self) -> TimeConditional {
    TimeConditional { 
      time_range: self.time_range, 
      weekday_set: self.weekday_set,
    }
  }
}

pub mod database {
  pub use crate::x::database::TimeConditionalSchema as Schema;
}
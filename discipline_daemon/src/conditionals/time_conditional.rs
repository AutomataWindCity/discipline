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
}
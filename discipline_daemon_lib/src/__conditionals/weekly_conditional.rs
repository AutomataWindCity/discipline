use crate::x::{Time, TimeRange, Weekday};
use serde::{Deserialize, Serialize};
use std::ops::{BitAnd, BitOr, BitOrAssign};

pub struct FiveMinuteBasedTime {
  inner: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiveMinuteIntervals {
  // 6 * 64 = 384 bits, covering 288 five-minute intervals
  bits: [u64; 6], 
}

impl FiveMinuteIntervals {
  // 288 five-minute intervals
  const INTERVALS_PER_DAY: usize = 24 * 12; 

  pub fn new() -> Self {
    Self { bits: [0; 6] }
  }

  pub fn set_interval(&mut self, interval: usize) {
    if interval < Self::INTERVALS_PER_DAY {
      let byte_index = interval / 64;
      let bit_index = interval % 64;
      self.bits[byte_index] |= 1 << bit_index;
    }
  }

  pub fn clear_interval(&mut self, interval: usize) {
    if interval < Self::INTERVALS_PER_DAY {
      let byte_index = interval / 64;
      let bit_index = interval % 64;
      self.bits[byte_index] &= !(1 << bit_index);
    }
  }

  pub fn is_interval_set(&self, interval: usize) -> bool {
    if interval < Self::INTERVALS_PER_DAY {
      let byte_index = interval / 64;
      let bit_index = interval % 64;
      (self.bits[byte_index] & (1 << bit_index)) != 0
    } else {
      false
    }
  }

  pub fn from_time(hour: u8, minute: u8) -> usize {
    ((hour as usize) * 12) + ((minute as usize) / 5)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyConditional {
  mon: FiveMinuteIntervals,
  tue: FiveMinuteIntervals,
  wed: FiveMinuteIntervals,
  thu: FiveMinuteIntervals,
  fri: FiveMinuteIntervals,
  sat: FiveMinuteIntervals,
  sun: FiveMinuteIntervals,
}

impl WeeklyConditional {
  pub fn is_activated(&self, time: Time, weekday: Weekday) -> bool {
    match weekday {
      Weekday::Mon => self.mon.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Tue => self.tue.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Wed => self.wed.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Thu => self.thu.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Fri => self.fri.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Sat => self.sat.is_interval_set(time.five_minute_timestamp_usize()),
      Weekday::Sun => self.sun.is_interval_set(time.five_minute_timestamp_usize()),
    }
  }

  pub fn add_range(&mut self, weekday: Weekday, time_range: TimeRange) {
    let weekdays = match weekday {
      Weekday::Fri => &mut self.fri,
      Weekday::Mon => &mut self.mon,
      Weekday::Sat => &mut self.sat,
      Weekday::Sun => &mut self.sun,
      Weekday::Thu => &mut self.thu,
      Weekday::Tue => &mut self.tue,
      Weekday::Wed => &mut self.wed,
    };

    // let position = weekdays.iter().position(|it| it.from().millisecond_timestamp() )
  }
}

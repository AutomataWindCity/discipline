use serde::{Deserialize, Serialize};
use crate::x::{DateTime, Duration, monotonic};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  pub remaining_duration: Duration,
  pub previous_synchronization_time: DateTime,
}

impl Countdown {
  pub fn new(duration: Duration, now: DateTime) -> Countdown {
    Countdown { 
      remaining_duration: duration,
      previous_synchronization_time: now, 
    }
  }

  pub fn from_fields(
    remaining_duration: Duration,
    previous_synchronization_time: DateTime,
  ) -> Countdown {
    Countdown {
      remaining_duration, 
      previous_synchronization_time 
    }
  }

  pub fn remaining_duration(&self) -> Duration {
    self.remaining_duration
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.previous_synchronization_time
  }

  pub fn is_finished(&self) -> bool {
    self.remaining_duration.is_zero()
  }

  pub fn synchronize(&mut self, now: DateTime) {
    let interval = self
      .previous_synchronization_time
      .till_or_zero(now);

    self.remaining_duration = self
      .remaining_duration
      .minus_or_zero(interval);

    self.previous_synchronization_time = now;
  }
}

pub trait IsCountdown {
  fn new(duration: Duration, now: DateTime) -> Self;

  fn get_remaining_duration(&self) -> Duration;
  fn set_remaining_duration(&mut self, new_value: Duration);
  fn get_previous_synchronization_time(&self) -> DateTime;
  fn set_previous_synchronization_time(&mut self, new_value: DateTime);

  fn is_finished(&self) -> bool {
    self.get_remaining_duration().is_zero()
  }

  fn synchronize(&mut self, now: DateTime) {
    let interval = self
      .get_previous_synchronization_time()
      .till_or_zero(now);

    self.set_remaining_duration(
      self
        .get_remaining_duration()
        .minus_or_zero(interval)
    );

    self.set_previous_synchronization_time(now);
  }
}

pub mod snapshoot {
  use crate::x::{Duration, DateTime, Countdown};

  pub struct Snapshoot {
    pub remaining_duration: Duration,
    pub previous_synchronization_time: DateTime,
  }

  impl Snapshoot {
    pub fn to_value(&self) -> Countdown {
      Countdown { 
        remaining_duration: self.remaining_duration, 
        previous_synchronization_time: self.previous_synchronization_time,
      }
    }
    pub fn apply_to(&self, countdown: &mut Countdown) {
      countdown.remaining_duration = self.remaining_duration;
      countdown.previous_synchronization_time = self.previous_synchronization_time;
    }
  }
}
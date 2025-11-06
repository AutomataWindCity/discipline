use serde::{Deserialize, Serialize};
use crate::x::{DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  remaining_duration: Duration,
  previous_synchronization_time: DateTime,
}

impl Countdown {
  pub fn new(duration: Duration, now: DateTime) -> Countdown {
    Countdown { 
      remaining_duration: duration,
      previous_synchronization_time: now, 
    }
  }

  pub fn construct(
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
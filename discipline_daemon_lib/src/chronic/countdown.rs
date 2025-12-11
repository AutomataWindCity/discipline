use serde::{Deserialize, Serialize};
use crate::x::{Duration, MonotonicInstant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  pub from: MonotonicInstant,
  pub duration: Duration,
}

impl Countdown {
  pub fn new(duration: Duration) -> Countdown {
    Countdown { 
      from: MonotonicInstant::MAX, 
      duration,
    }
  }

  pub fn construct(from: MonotonicInstant, duration: Duration) -> Countdown {
    Countdown { 
      from, 
      duration,
    }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn remaining_duration(&self, now: MonotonicInstant) -> Duration {
    self.from.till_or_zero(now).minus_or_zero(self.duration)
  }

  pub fn is_finished(&self, now: MonotonicInstant) -> bool {
    self.remaining_duration(now).is_zero()
  }

  pub fn is_running(&self, now: MonotonicInstant) -> bool {
    !self.remaining_duration(now).is_zero()
  }

  pub fn begin(&mut self, now: MonotonicInstant) {
    self.from = now;
  }

  pub fn cancel(&mut self) {
    self.from = MonotonicInstant::MAX;
  }
}
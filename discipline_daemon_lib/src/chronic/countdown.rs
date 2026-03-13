use serde::{Deserialize, Serialize};
use crate::x::{Duration, MonotonicInstant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownStatus {
  Pending,
  Running,
  Finished,
}

impl CountdownStatus {
  pub fn is_pending(self) -> bool {
    matches!(self, CountdownStatus::Pending)
  }

  pub fn is_running(self) -> bool {
    matches!(self, CountdownStatus::Running)
  }

  pub fn is_finished(self) -> bool {
    matches!(self, CountdownStatus::Finished)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  pub from: MonotonicInstant,
  pub duration: Duration,
}

impl Countdown {
  pub fn create(from: MonotonicInstant, duration: Duration) -> Self {
    Self {
      from,
      duration,
    }
  }

  pub fn construct(from: MonotonicInstant, duration: Duration) -> Countdown {
    Self { 
      from, 
      duration,
    }
  }

  pub fn status(&self, now: MonotonicInstant) -> CountdownStatus {
    if now.is_eariler_than(self.from) {
      return CountdownStatus::Pending;
    } 
    
    let elapsed_time = self.from.till_or_zero(now);
    if elapsed_time.is_shorter_than_or_equal_to(self.duration) {
      return CountdownStatus::Running;
    }
    
    CountdownStatus::Finished
  }

  pub fn remaining_duration(&self, now: MonotonicInstant) -> Duration {
    match self.status(now) {
      CountdownStatus::Pending => {
        self.duration
      }
      CountdownStatus::Running => {
        self.duration.minus_or_zero(self.from.till_or_zero(now))
      }
      CountdownStatus::Finished => {
        Duration::zero()
      }
    }
  }

  pub fn is_pending(&self, now: MonotonicInstant) -> bool {
    self.status(now).is_pending()
  }

  pub fn is_running(&self, now: MonotonicInstant) -> bool {
    self.status(now).is_running()
  }

  pub fn is_finished(&self, now: MonotonicInstant) -> bool {
    self.status(now).is_finished()
  }
}
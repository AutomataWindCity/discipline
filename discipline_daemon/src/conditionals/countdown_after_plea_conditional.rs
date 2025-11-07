use serde::{Deserialize, Serialize};
use crate::x::{Countdown, DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
  Activated,
  Deactivating { countdown: Countdown },
  Deactivated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownAfterPleaConditional {
  duration: Duration,
  status: Status,
}

impl CountdownAfterPleaConditional {
  pub fn new(duration: Duration) -> Self {
    Self {
      duration,
      status: Status::Deactivated
    }
  }

  pub fn from_fields(
    duration: Duration,
    status: Status,
  ) -> Self {
    Self {
      duration,
      status,
    }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn status(&self) -> &Status {
    &self.status
  }

  pub fn evaluate(&self) -> bool {
    match self.status {
      Status::Activated => true,
      Status::Deactivating { .. } => true,
      Status::Deactivated => false,
    }
  }

  pub fn synchronize(&mut self, now: DateTime) {
    if let Status::Deactivating { countdown } = &mut self.status {
      countdown.synchronize(now);
      if countdown.is_finished() {
        self.status = Status::Deactivated
      }
    }
  }

  pub fn activate(&mut self) {
    self.status = Status::Activated;
  }

  pub fn deactivate(&mut self, now: DateTime) {
    if let Status::Activated = self.status {
      self.status = Status::Deactivating { 
        countdown: Countdown::new(self.duration, now),
      };
    }
  }
}
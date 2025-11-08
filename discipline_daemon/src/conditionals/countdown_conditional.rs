use serde::{Deserialize, Serialize};
use crate::x::{Countdown, DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditional {
  duration: Duration,
  countdown: Option<Countdown>
}

impl CountdownConditional {
  pub fn new(duration: Duration) -> Self {
    Self {
      duration,
      countdown: None,
    }
  }

  pub fn from_fields(
    duration: Duration, 
    countdown: Option<Countdown>,
  ) -> Self {
    Self {
      duration,
      countdown: countdown.and_then(|countdown| if countdown.is_finished() {
        None
      } else {
        Some(countdown)
      })
    }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn countdown(&self) -> &Option<Countdown> {
    &self.countdown
  }

  pub fn activate(&mut self, now: DateTime) {
    self.countdown = Some(Countdown::new(
      self.duration,
      now
    ));
  }

  pub fn evaluate(&self) -> bool {
    self.countdown.is_some()
  }

  pub fn synchronize(&mut self, now: DateTime) {
    if let Some(countdown) = &mut self.countdown {
      countdown.synchronize(now);
      if countdown.is_finished() {
        self.countdown = None;
      }
    }
  }
}
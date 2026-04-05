use serde::{Deserialize, Serialize};
use crate::x::{Countdown, Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditional {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}

impl CountdownConditional {
  pub fn create(duration: Duration) -> CountdownConditional {
    CountdownConditional { duration, countdown: None }
  }
  
  pub fn is_active(&self, now: Instant) -> bool {
    matches!(&self.countdown, Some(countdown) if countdown.is_running(now))
  }

  pub fn activate(&mut self, now: Instant) {
    self.countdown = Some(Countdown::construct(now, self.duration));
  }

  pub fn create_activate_state(&self, now: Instant) -> CountdownConditionalActivateState {
    CountdownConditionalActivateState { countdown: Countdown::construct(now, self.duration) }
  }

  pub fn activate_from_activate_state(&mut self, activate_state: CountdownConditionalActivateState) {
    self.countdown = Some(activate_state.countdown);
  }
}

pub struct CountdownConditionalActivateState {
  pub countdown: Countdown,
}
use serde::{Deserialize, Serialize};
use crate::x::{Countdown, CountdownState, Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownAfterPleaConditional {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownAfterPleaConditionalState {
  Active,
  Deactivating,
  Deactivated,
}

impl CountdownAfterPleaConditionalState {
  pub fn is_active(self) -> bool {
    matches!(self, Self::Active)
  }
  
  pub fn is_deactivaing(self) -> bool {
    matches!(self, Self::Deactivating)
  }
  
  pub fn is_deactivated(self) -> bool {
    matches!(self, Self::Deactivated)
  }

  pub fn is_activate_or_deactivating(self) -> bool {
    matches!(self, Self::Active | Self::Deactivating)
  }
}

impl CountdownAfterPleaConditional {
  pub fn create(duration: Duration) -> Self {
    Self {
      duration,
      countdown: None,
    }
  }

  pub fn construct(duration: Duration, countdown: Option<Countdown>) -> Self {
    Self { 
      duration,
      countdown,
    }
  }
  
  pub fn get_state(&self, now: Instant) -> CountdownAfterPleaConditionalState {
    let Some(countdown) = &self.countdown else {
      return CountdownAfterPleaConditionalState::Active;
    };

    match countdown.get_state(now) {
      CountdownState::Pending => {
        CountdownAfterPleaConditionalState::Active
      }
      CountdownState::Running => {
        CountdownAfterPleaConditionalState::Deactivating
      }
      CountdownState::Finished => {
        CountdownAfterPleaConditionalState::Deactivated
      }
    }
  }

  pub fn is_active(&self, now: Instant) -> bool {
    self.get_state(now).is_active()
  }

  pub fn is_deactivated(&self, now: Instant) -> bool {
    self.get_state(now).is_deactivated()
  }

  pub fn is_activate_or_deactivating(&self, time: Instant) -> bool {
    matches!(
      self.get_state(time), 
      CountdownAfterPleaConditionalState::Active 
      | 
      CountdownAfterPleaConditionalState::Deactivating,
    )
  }

  pub fn activate(&mut self) {
    self.countdown = None;
  }

  pub fn deactivate(&mut self, now: Instant) {
    self.countdown = Some(Countdown::construct(now, self.duration))
  }

  pub fn create_deactivating_state(&self, now: Instant) -> CountdownAfterPleaConditionalDeactivatingState {
    CountdownAfterPleaConditionalDeactivatingState {
      countdown: Countdown::create(now, self.duration)
    }
  }
}

pub struct CountdownAfterPleaConditionalDeactivatingState {
  pub countdown: Countdown,
}
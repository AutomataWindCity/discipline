use serde::{Deserialize, Serialize};
use crate::x::{Countdown, DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditional {
  pub duration: Duration,
  pub countdown: Option<Countdown>
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
  
  pub fn countdown2(&self) -> Option<&Countdown> {
    self.countdown.as_ref()
  }

  pub fn activate(&mut self, now: DateTime) {
    self.countdown = Some(Countdown::new(
      self.duration,
      now
    ));
  }

  pub fn is_activated(&self) -> bool {
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

pub mod operations {
  use serde::{Serialize, Deserialize};
  use crate::x::{CountdownConditional, DateTime};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Activate;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum ActivateReturn {
    Success,
    AlreadyActivated,
  }

  impl Activate {
    pub fn execute(
      self,
      conditional: &mut CountdownConditional, 
    ) -> ActivateReturn {
      if conditional.is_activated() {
        return ActivateReturn::AlreadyActivated;
      }

      conditional.activate(DateTime::now());
      ActivateReturn::Success
    }
  }

  pub enum Call {
    Activate(Activate),
  }

  pub enum CallReturn {
    Activate(ActivateReturn),
  }

  impl Call {
    pub fn execute(
      self,
      conditional: &mut CountdownConditional,
    ) -> CallReturn {
      match self {
        Call::Activate(call) => {
          CallReturn::Activate(call.execute(conditional))
        }
      }
    }
  }
}
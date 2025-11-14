use serde::{Deserialize, Serialize};
use crate::x::{CountdownX, InstantX, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownAfterPleaConditionalX {
  is_activated: bool,
  countdown: CountdownX,
}

impl CountdownAfterPleaConditionalX {
  pub fn new(duration: Duration) -> Self {
    Self {
      is_activated: false,
      countdown: CountdownX::new(duration)
    }
  }

  pub fn construct(is_activated: bool, countdown: CountdownX) -> Self {
    Self { is_activated, countdown }
  }

  pub fn is_activated(&self) -> bool {
    self.is_activated
  }

  pub fn is_activated_or_deactivating(&self, now: InstantX) -> bool {
    self.is_activated || self.countdown.is_running(now)
  }

  pub fn is_deactivaing(&self, now: InstantX) -> bool {
    !self.is_activated && self.countdown.is_running(now)
  }

  pub fn is_deactivated(&self, now: InstantX) -> bool {
    !self.is_activated && self.countdown.is_finished(now)
  }

  pub fn activate(&mut self) {
    self.is_activated = true;
    self.countdown.cancel();
  }

  pub fn deactivate(&mut self, now: InstantX) {
    self.is_activated = true;
    self.countdown.begin(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  duration: Duration,
}

impl Creator {
  pub fn create(self) -> CountdownAfterPleaConditionalX {
    CountdownAfterPleaConditionalX::new(self.duration)
  }
}

pub mod snapshoot {
  use crate::x::{countdown_x, CountdownAfterPleaConditionalX};

  pub struct Snapshoot {
    is_activated: bool,
    countdown: countdown_x::snapshoot::Snapshoot,
  }

  impl Snapshoot {
    pub fn revert_changes(&self, conditional: &mut CountdownAfterPleaConditionalX) {
      conditional.is_activated = self.is_activated;
      self.countdown.revert_changes(&mut conditional.countdown);
    }
  }
}

pub mod procedures {
  use serde::{Serialize, Deserialize};
  use crate::x::{InstantX, CountdownAfterPleaConditionalX};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Activate;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum ActivateReturn {
    AlreadyActivated,
    Success,
  }

  impl Activate {
    pub fn execute(
      self,
      instant: InstantX,
      conditional: &mut CountdownAfterPleaConditionalX,
    ) -> ActivateReturn {
      if conditional.is_activated_or_deactivating(instant) {
        return ActivateReturn::AlreadyActivated;
      }

      conditional.activate();
      ActivateReturn::Success
    }
  }
  
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Deactivate;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum DeactivateReturn {
    AlreadyDeactivated,
    AlreadyDeactivating,
    Success,
  }

  impl Deactivate {
    pub fn execute(
      self, 
      instant: InstantX,
      conditional: &mut CountdownAfterPleaConditionalX,
    ) -> DeactivateReturn {
      if conditional.is_deactivated(instant) {
        return DeactivateReturn::AlreadyDeactivated;
      }
      if conditional.is_deactivaing(instant) {
        return DeactivateReturn::AlreadyDeactivating;
      }

      conditional.deactivate(instant);
      DeactivateReturn::Success
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum Procedure {
    Activate(Activate),
    Deactivate(Deactivate),
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum Return {
    Activate(ActivateReturn),
    Deactivate(DeactivateReturn),
  }

  impl Procedure {
    pub fn execute(
      self, 
      instant: InstantX,
      conditional: &mut CountdownAfterPleaConditionalX,
    ) -> Return {
      match self {
        Procedure::Activate(call) => {
          Return::Activate(call.execute(instant, conditional))
        }
        Procedure::Deactivate(call) => {
          Return::Deactivate(call.execute(instant, conditional))
        }
      }
    }
  }
}
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
  status: Status,
  duration: Duration,
}

impl CountdownAfterPleaConditional {
  pub fn new(duration: Duration) -> Self {
    Self {
      duration,
      status: Status::Deactivated
    }
  }

  pub fn construct(
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

  pub fn is_activated_or_deactivating(&self) -> bool {
    match self.status {
      Status::Activated => true,
      Status::Deactivating { .. } => true,
      Status::Deactivated => false,
    }
  }

  pub fn is_deactivaing(&self) -> bool {
    matches!(self.status, Status::Deactivating { .. })
  }

  pub fn is_deactivated(&self) -> bool {
    matches!(self.status, Status::Deactivated)
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

pub mod snapshoot {
  use crate::x::{Duration, __countdown};
  use crate::x::countdown_after_plea_conditional::{CountdownAfterPleaConditional, Status};

  enum StatusSnapsoot {
    Activated,
    Deactivating { countdown: __countdown::snapshoot::Snapshoot },
    Deactivated,
  }

  impl StatusSnapsoot {
    fn to_value(&self) -> Status {
      match self {
        Self::Activated => Status::Activated,
        Self::Deactivated => Status::Deactivated,
        Self::Deactivating { countdown } => Status::Deactivating { countdown: countdown.to_value() }
      }
    }
  }
  
  pub struct Snapshoot {
    status: StatusSnapsoot,
  }

  impl Snapshoot {
    pub fn apply_to(&self, conditional: &mut CountdownAfterPleaConditional) {
      match (&self.status, &mut conditional.status) {
        (StatusSnapsoot::Activated, Status::Activated) => {
          // no operation
        }
        (StatusSnapsoot::Deactivated, Status::Deactivated) => {
          // no operation
        }
        (StatusSnapsoot::Deactivating { countdown: snapshoot }, Status::Deactivating { countdown }) => {
          snapshoot.apply_to(countdown);
        }
        (status_snapshoot, _) => {
          conditional.status = status_snapshoot.to_value();
        }
      }
    }
  }
}

pub mod operations {
  use serde::{Serialize, Deserialize};
  use crate::x::{DateTime, CountdownAfterPleaConditional};

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
      conditional: &mut CountdownAfterPleaConditional,
    ) -> ActivateReturn {
      if conditional.is_activated_or_deactivating() {
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
      conditional: &mut CountdownAfterPleaConditional,
    ) -> DeactivateReturn {
      if conditional.is_deactivated() {
        return DeactivateReturn::AlreadyDeactivated;
      }
      if conditional.is_deactivaing() {
        return DeactivateReturn::AlreadyDeactivating;
      }

      conditional.deactivate(DateTime::now());
      DeactivateReturn::Success
    }
  }

  pub enum Call {
    Activate(Activate),
    Deactivate(Deactivate),
  }

  pub enum CallReturn {
    Activate(ActivateReturn),
    Deactivate(DeactivateReturn),
  }

  impl Call {
    pub fn execute(
      self, 
      conditional: &mut CountdownAfterPleaConditional,
    ) -> CallReturn {
      match self {
        Call::Activate(call) => {
          CallReturn::Activate(call.execute(conditional))
        }
        Call::Deactivate(call) => {
          CallReturn::Deactivate(call.execute(conditional))
        }
      }
    }
  }
}
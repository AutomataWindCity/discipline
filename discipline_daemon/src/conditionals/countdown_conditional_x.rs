use serde::{Deserialize, Serialize};
use crate::x::{CountdownX, Duration, countdown_x, time_x};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditionalX {
  pub countdown: countdown_x::CountdownX,
}

impl CountdownConditionalX {
  pub fn new(duration: Duration) -> Self {
    Self {
      countdown: CountdownX::new(duration),
    }
  }

  pub fn construct(countdown: CountdownX) -> Self {
    Self { countdown }
  }

  pub fn countdown(&self) -> &CountdownX {
    &self.countdown
  }

  pub fn is_activated(&self, now: time_x::InstantX) -> bool {
    self.countdown.is_running(now)
  }

  pub fn activate(&mut self, now: time_x::InstantX) {
    self.countdown.begin(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  duration: Duration,
}

impl Creator {
  pub fn create(self) -> CountdownConditionalX {
    CountdownConditionalX::new(self.duration)
  }
}

pub mod snapshoot {
  use crate::x::{countdown_x, CountdownConditionalX};

  pub struct Snapshoot {
    countdown: countdown_x::snapshoot::Snapshoot,
  }

  impl Snapshoot {
    pub fn revert_changes(&self, conditional: &mut CountdownConditionalX) {
      self.countdown.revert_changes(&mut conditional.countdown);
    }
  }
}

pub mod operations {
  use serde::{Serialize, Deserialize};
  use crate::x::{InstantX, CountdownConditionalX};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Activate;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum ActivateReturn {
    AlreadyActivated,
    Success,
  }

  impl Activate {
    pub fn execute(self, instant: InstantX, conditional: &mut CountdownConditionalX) -> ActivateReturn {
      if conditional.is_activated(instant) {
        return ActivateReturn::AlreadyActivated;
      }

      conditional.activate(instant);
      ActivateReturn::Success
    }
  }

  pub enum Operation {
    Activate(Activate),
  }

  pub enum OperationReturn {
    Activate(ActivateReturn),
  }

  impl Operation {
    pub fn execute(
      self, 
      instant: InstantX, 
      conditional: &mut CountdownConditionalX,
    ) -> OperationReturn {
      match self {
        Operation::Activate(operation) => {
          OperationReturn::Activate(operation.execute(instant, conditional))
        }
      }
    }
  }
}

pub mod database {
  
}
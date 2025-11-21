use serde::{Deserialize, Serialize};
use crate::x::{Countdown, Duration, countdown, monotonic};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditional {
  pub countdown: countdown::Countdown,
}

impl CountdownConditional {
  pub fn new(duration: Duration) -> Self {
    Self {
      countdown: Countdown::new(duration),
    }
  }

  pub fn construct(countdown: Countdown) -> Self {
    Self { countdown }
  }

  pub fn countdown(&self) -> &Countdown {
    &self.countdown
  }

  pub fn is_activated(&self, now: monotonic::MonotonicInstant) -> bool {
    self.countdown.is_running(now)
  }

  pub fn activate(&mut self, now: monotonic::MonotonicInstant) {
    self.countdown.begin(now);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  duration: Duration,
}

impl Creator {
  pub fn create(self) -> CountdownConditional {
    CountdownConditional::new(self.duration)
  }
}

pub mod procedures {
  use serde::{Serialize, Deserialize};
  use crate::x::{MonotonicInstant, CountdownConditional};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Activate;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum ActivateSuccess {
    AlreadyActivated,
    Success,
  }

  impl Activate {
    pub fn execute(
      self, 
      now: MonotonicInstant, 
      conditional: &mut CountdownConditional,
    ) -> ActivateSuccess {
      if conditional.is_activated(now) {
        return ActivateSuccess::AlreadyActivated;
      }

      conditional.activate(now);
      ActivateSuccess::Success
    }
  }

  pub enum Procedure {
    Activate(Activate),
  }

  pub enum ProcedureSuccess {
    Activate(ActivateSuccess),
  }

  impl Procedure {
    pub fn execute(
      self, 
      now: MonotonicInstant, 
      conditional: &mut CountdownConditional,
    ) -> ProcedureSuccess {
      match self {
        Procedure::Activate(operation) => {
          ProcedureSuccess::Activate(operation.execute(now, conditional))
        }
      }
    }
  }
}

pub mod database {
  pub use crate::x::database::CountdownConditionalSchema as Schema;
}
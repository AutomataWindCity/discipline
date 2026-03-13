use serde::{Deserialize, Serialize};
use crate::x::{Countdown, Duration, MonotonicInstant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownConditional {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}

impl CountdownConditional {
  pub fn create(duration: Duration, countdown: Option<Countdown>) -> Self {
    Self {
      duration,
      countdown,
    }
  }

  pub fn construct(duration: Duration, countdown: Option<Countdown>) -> Self {
    Self {
      duration,
      countdown,
    }
  }

  pub fn is_activated(&self, now: MonotonicInstant) -> bool {
    match &self.countdown {
      Some(countdown) if countdown.is_running(now) => {
        true
      }
      _ => {
        false
      }
    }
  }

  pub fn activate(&mut self, now: MonotonicInstant) {
    self.countdown = Some(Countdown::construct(
      now, 
      self.duration,
    ));
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  duration: Duration,
}

impl Creator {
  pub fn create(self) -> CountdownConditional {
    CountdownConditional::create(
      self.duration, 
      None,
    )
  }
}

// pub mod procedures {
//   use serde::{Serialize, Deserialize};
//   use crate::x::{MonotonicInstant, CountdownConditional};

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub struct Activate;

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum ActivateSuccess {
//     AlreadyActivated,
//     Success,
//   }

//   impl Activate {
//     pub fn execute(
//       self, 
//       now: MonotonicInstant, 
//       conditional: &mut CountdownConditional,
//     ) -> ActivateSuccess {
//       if conditional.is_activated(now) {
//         return ActivateSuccess::AlreadyActivated;
//       }

//       conditional.activate(now);
//       ActivateSuccess::Success
//     }
//   }

//   pub enum Procedure {
//     Activate(Activate),
//   }

//   pub enum ProcedureSuccess {
//     Activate(ActivateSuccess),
//   }

//   impl Procedure {
//     pub fn execute(
//       self, 
//       now: MonotonicInstant, 
//       conditional: &mut CountdownConditional,
//     ) -> ProcedureSuccess {
//       match self {
//         Procedure::Activate(operation) => {
//           ProcedureSuccess::Activate(operation.execute(now, conditional))
//         }
//       }
//     }
//   }
// }

pub mod database {
  pub use crate::x::database::CountdownConditionalSchema as Schema;
}
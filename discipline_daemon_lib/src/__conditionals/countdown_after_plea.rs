use serde::{Deserialize, Serialize};
use crate::x::{Countdown, CountdownStatus, Instant, Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownAfterPleaConditionalStatus {
  Active,
  Deactivating,
  Deactivated,
}

impl CountdownAfterPleaConditionalStatus {
  pub fn is_active(&self) -> bool {
    matches!(self, Self::Active)
  }
  
  pub fn is_deactivaing(&self) -> bool {
    matches!(self, Self::Deactivating)
  }
  
  pub fn is_deactivated(&self) -> bool {
    matches!(self, Self::Deactivated)
  }
}

/// A conditional that is always 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownAfterPleaConditional {
  pub duration: Duration,
  pub countdown: Option<Countdown>,
}

impl CountdownAfterPleaConditional {
  pub fn new(duration: Duration) -> Self {
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

  pub fn countdown(&self) -> &Option<Countdown> {
    &self.countdown
  }
  
  pub fn status(&self, now: Instant) -> CountdownAfterPleaConditionalStatus {
    let Some(countdown) = &self.countdown else {
      return CountdownAfterPleaConditionalStatus::Active;
    };

    match countdown.get_status(now) {
      CountdownStatus::Pending => {
        CountdownAfterPleaConditionalStatus::Active
      }
      CountdownStatus::Running => {
        CountdownAfterPleaConditionalStatus::Deactivating
      }
      CountdownStatus::Finished => {
        CountdownAfterPleaConditionalStatus::Deactivated
      }
    }
  }

  pub fn activate(&mut self) {
    self.countdown = None;
  }

  pub fn deactivate(&mut self, now: Instant) {
    self.countdown = Some(Countdown::construct(now, self.duration))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  duration: Duration,
}

impl Creator {
  pub fn create(self) -> CountdownAfterPleaConditional {
    CountdownAfterPleaConditional::new(self.duration)
  }
}

// pub mod procedures {
//   use serde::{Serialize, Deserialize};
//   use crate::x::{Instant, CountdownAfterPleaConditionalX};

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub struct Activate;

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum ActivateReturn {
//     AlreadyActivated,
//     Success,
//   }

//   impl Activate {
//     pub fn execute(
//       self,
//       instant: Instant,
//       conditional: &mut CountdownAfterPleaConditionalX,
//     ) -> ActivateReturn {
//       if conditional.is_activated_or_deactivating(instant) {
//         return ActivateReturn::AlreadyActivated;
//       }

//       conditional.activate();
//       ActivateReturn::Success
//     }
//   }
  
//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub struct Deactivate;

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum DeactivateReturn {
//     AlreadyDeactivated,
//     AlreadyDeactivating,
//     Success,
//   }

//   impl Deactivate {
//     pub fn execute(
//       self, 
//       instant: Instant,
//       conditional: &mut CountdownAfterPleaConditionalX,
//     ) -> DeactivateReturn {
//       if conditional.is_deactivated(instant) {
//         return DeactivateReturn::AlreadyDeactivated;
//       }
//       if conditional.is_deactivaing(instant) {
//         return DeactivateReturn::AlreadyDeactivating;
//       }

//       conditional.deactivate(instant);
//       DeactivateReturn::Success
//     }
//   }

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum Procedure {
//     Activate(Activate),
//     Deactivate(Deactivate),
//   }

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum Return {
//     Activate(ActivateReturn),
//     Deactivate(DeactivateReturn),
//   }

//   impl Procedure {
//     pub fn execute(
//       self, 
//       instant: Instant,
//       conditional: &mut CountdownAfterPleaConditionalX,
//     ) -> Return {
//       match self {
//         Procedure::Activate(call) => {
//           Return::Activate(call.execute(instant, conditional))
//         }
//         Procedure::Deactivate(call) => {
//           Return::Deactivate(call.execute(instant, conditional))
//         }
//       }
//     }
//   }
// }
pub mod database {
  pub use crate::x::database::CountdownAfterPleaConditionalSchema as Schema;
}
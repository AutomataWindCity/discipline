use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, conditionals};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleActivatorCreator {
  Time(conditionals::time::Creator),
  Always(conditionals::always::Creator)
}

impl RuleActivatorCreator {
  pub fn create(self) -> RuleActivator {
    match self {
      Self::Time(creator) => {
        RuleActivator::Time(creator.create())
      }
      Self::Always(creator) => {
        RuleActivator::Always(creator.create())
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnablerCreator {
  Countdown(conditionals::countdown::Creator),
  CountdownAfterPlea(conditionals::countdown_after_plea::Creator)
}

impl RuleEnablerCreator {
  pub fn create(self) -> RuleEnabler {
    match self {
      Self::Countdown(creator) => {
        RuleEnabler::Countdown(creator.create())
      }
      Self::CountdownAfterPlea(creator) => {
        RuleEnabler::CountdownAfterPlea(creator.create())
      }
    }
  }
}

// TODO: This is not used anywhere. Delete it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreator {
  pub id: Option<UuidV4>,
  pub activator: RuleActivatorCreator,
  pub enabler: RuleEnablerCreator,
}

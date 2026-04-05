use serde::{Deserialize, Serialize};
use crate::x::TextualError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleEnablerType {
  Countdown,
  CountdownAfterPlea,
}

impl RuleEnablerType {
  const COUNTDOWN_AS_NUMBER: u8 = 0;
  const COUNTDOWN_AFTER_PLEA_AS_NUMBER: u8 = 0;

  pub fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::COUNTDOWN_AS_NUMBER => {
        Ok(Self::Countdown)
      }
      Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER => {
        Ok(Self::CountdownAfterPlea)
      }
      _ => {
        Err(TextualError::new("action"))
      }
    }
  }

  pub fn to_number(self) -> u8 {
    match self {
      RuleEnablerType::Countdown => {
        Self::COUNTDOWN_AS_NUMBER
      }
      RuleEnablerType::CountdownAfterPlea => {
        Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER
      }
    }
  }
}

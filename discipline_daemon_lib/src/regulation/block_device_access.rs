use serde::{Deserialize, Serialize};
use crate::x::{CountdownConditional, WeeklyConditional};

pub struct ProfileName {
  string: String,
}

pub enum CreateProfileNameError {
  LengthViolation { string: String },
}

impl ProfileName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<Self, CreateProfileNameError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Err(CreateProfileNameError::LengthViolation { string });
    }
    if string.len() < Self::MAXIMUM_LENGTH {
      return Err(CreateProfileNameError::LengthViolation { string });
    }
    Ok(Self { string })
  }
}

pub struct Profile {
  pub allow_for: CountdownConditional,
  pub block_for: CountdownConditional,
  pub block_when: WeeklyConditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesManager {
  profile_naumber: usize,
  maximum_profile_naumber: usize,
}

impl ProfilesManager {
  pub fn new() -> Self {
    Self {
      profile_naumber: 0,
      maximum_profile_naumber: 0,
    }
  }
}

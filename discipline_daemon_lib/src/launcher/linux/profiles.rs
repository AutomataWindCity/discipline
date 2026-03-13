use std::any::type_name;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, AlwaysRules, TextualErrorContext, TimeRangeRules, ToTextualError};
use super::{UserId, UserName};

#[derive(Debug, Clone)]
pub struct UserProfileName {
  inner: String,
}

#[derive(Debug, Clone)]
pub enum CreateFromStringError {
  LengthViolation { string: String },
}

impl ToTextualError for CreateFromStringError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new(format!("Creating {} from String", type_name::<UserProfileName>()));
   
    match self {
      Self::LengthViolation { string } => {
        context.add_message("String length is invalid");
        context.add_attachement_display("Minimum valid length", UserProfileName::MINIMUM_LENGTH);
        context.add_attachement_display("Maximum valid length", UserProfileName::MAXIMUM_LENGTH);
        context.add_attachement_display("Found string length", string.len());
        context.add_attachement_display("String", string);
      }
    }

    context
  }
}

impl UserProfileName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<UserProfileName, CreateFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Err(CreateFromStringError::LengthViolation { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Err(CreateFromStringError::LengthViolation { string });
    }
    Ok(Self { inner: string })
  }

  pub fn as_str(&self) -> &str {
    &self.inner
  }

  pub fn as_string(&self) -> &String {
    &self.inner
  }
}

// deny_device_access_time_rules
// deny_device_access_always_rules

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAccessRegulation {
  always_rules: AlwaysRules,
  time_range_rules: TimeRangeRules,
  // uptime_allowance_rules,
}

impl DeviceAccessRegulation {
  pub fn new() -> Self {
    Self {
      always_rules: AlwaysRules::new(),
      time_range_rules: TimeRangeRules::new(),
    }
  }
  
  pub fn construct(
    always_rules: AlwaysRules,
    time_range_rules: TimeRangeRules,
  ) -> Self {
    Self {
      always_rules,
      time_range_rules,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenAccessRegulation {
  always_rules: AlwaysRules,
  time_range_rules: TimeRangeRules,
  // screen_time_allowance_rules
}

impl ScreenAccessRegulation {
  pub fn new() -> Self {
    Self {
      always_rules: AlwaysRules::new(),
      time_range_rules: TimeRangeRules::new(),
    }
  }
  
  pub fn construct(
    always_rules: AlwaysRules,
    time_range_rules: TimeRangeRules,
  ) -> Self {
    Self {
      always_rules,
      time_range_rules,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetAccessRegulation {
  always_rules: AlwaysRules,
  time_range_rules: TimeRangeRules,
  // traffic_allowance_rules
}

impl InternetAccessRegulation {
  pub fn new() -> Self {
    Self {
      always_rules: AlwaysRules::new(),
      time_range_rules: TimeRangeRules::new(),
    }
  }

  pub fn construct(
    always_rules: AlwaysRules,
    time_range_rules: TimeRangeRules,
  ) -> Self {
    Self {
      always_rules,
      time_range_rules,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
  name: UserProfileName,
  user_id: UserId,
  user_name: UserName,
  device_access_regulation: DeviceAccessRegulation,
  screen_access_regulation: ScreenAccessRegulation,
  internet_access_regulation: InternetAccessRegulation,
}

impl UserProfile {
  pub fn new(
    name: UserProfileName,
    user_id: UserId,
    user_name: UserName,
  ) -> Self {
    Self {
      name,
      user_id,
      user_name,
      device_access_regulation: DeviceAccessRegulation::new(),
      screen_access_regulation: ScreenAccessRegulation::new(),
      internet_access_regulation: InternetAccessRegulation::new(),
    }
  } 
  
  pub fn construct(
    name: UserProfileName,
    user_id: UserId,
    user_name: UserName,
    device_access_regulation: DeviceAccessRegulation,
    screen_access_regulation: ScreenAccessRegulation,
    internet_access_regulation: InternetAccessRegulation,
  ) -> Self {
    Self {
      name,
      user_id,
      user_name,
      device_access_regulation,
      screen_access_regulation,
      internet_access_regulation,
    }
  } 
}

pub struct UserProfilesStats {
  maximum_user_number: usize,
}

impl Default for UserProfilesStats {
  fn default() -> Self {
    Self {
      maximum_user_number: 50,
    }
  }
}

impl UserProfilesStats {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      maximum_user_number,
    }
  }

  pub fn construct(maximum_user_number: usize) -> Self {
    Self {
      maximum_user_number,
    }
  }

  pub fn get_maximum_user_number(&self) -> usize {
    self.maximum_user_number
  }
}

#[derive(Debug)]
pub struct UserProfiles {
  user_profiles: HashMap<UuidV4, User>,
}

impl UserProfiles {
  pub fn new() -> Self {
    Self {
      user_profiles: HashMap::new(),
    }
  }

  pub fn construct(user_profiles: HashMap<UuidV4, User>) -> Self {
    Self {
      user_profiles,
    }
  }

  pub fn get_user(&self, user_id: &UuidV4) -> Option<&User> {
    self.user_profiles.get(user_id)
  }

  pub fn get_users_number(&self) -> usize {
    self.user_profiles.len()
  }

  pub fn add_user(&mut self, user_id: UuidV4, user: User) {
    self.user_profiles.insert(user_id, user);
  }

  pub fn delete_user(&mut self, user_id: &UuidV4) {
    self.user_profiles.remove(user_id);
  }

  pub fn contains_user(&self, user_id: &UuidV4) -> bool {
    self.user_profiles.contains_key(user_id)
  }
}
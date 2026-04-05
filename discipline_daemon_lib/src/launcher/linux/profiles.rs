use std::any::type_name;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::x::{AlwaysRules, Instant, RulesStats, TextualErrorContext, Time, TimeAllowanceRules, TimeRangeRules, ToTextualError, UserUptimeClock, UuidV4};
use super::{UserId, UserName};


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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileName {
  inner: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceAccessRegulation {
  pub always_rules: AlwaysRules,
  pub time_range_rules: TimeRangeRules,
  pub daily_uptime_allowance_rules: TimeAllowanceRules,
  pub weekly_uptime_allowance_rules: TimeAllowanceRules,
}

impl DeviceAccessRegulation {
  pub fn new() -> Self {
    Self {
      always_rules: AlwaysRules::default(),
      time_range_rules: TimeRangeRules::default(),
      daily_uptime_allowance_rules: TimeAllowanceRules::default(),
      weekly_uptime_allowance_rules: TimeAllowanceRules::default(),
    }
  }
  
  pub fn construct(
    always_rules: AlwaysRules,
    time_range_rules: TimeRangeRules,
    daily_uptime_allowance_rules: TimeAllowanceRules,
    weekly_uptime_allowance_rules: TimeAllowanceRules,
  ) -> Self {
    Self {
      always_rules,
      time_range_rules,
      daily_uptime_allowance_rules,
      weekly_uptime_allowance_rules,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreenAccessRegulation {
  pub always_rules: AlwaysRules,
  pub time_range_rules: TimeRangeRules,
  pub daily_allowance_rules: TimeAllowanceRules,
  pub weekly_allowance_rules: TimeAllowanceRules,
}

impl ScreenAccessRegulation {
  pub fn construct(
    always_rules: AlwaysRules,
    time_range_rules: TimeRangeRules,
    daily_allowance_rules: TimeAllowanceRules,
    weekly_allowance_rules: TimeAllowanceRules,
  ) -> Self {
    Self {
      always_rules,
      time_range_rules,
      daily_allowance_rules,
      weekly_allowance_rules
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetAccessRegulation {
  pub always_rules: AlwaysRules,
  pub time_range_rules: TimeRangeRules,
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
  pub name: UserProfileName,
  pub user_id: UserId,
  pub user_name: UserName,
  pub uptime_clock: UserUptimeClock,
  pub device_access_regulation: DeviceAccessRegulation,
  pub screen_access_regulation: ScreenAccessRegulation,
  pub internet_access_regulation: InternetAccessRegulation,
  pub rules_stats: RulesStats,
}

impl UserProfile {
  pub fn new(
    name: UserProfileName,
    user_id: UserId,
    user_name: UserName,
    uptime_clock: UserUptimeClock,
  ) -> Self {
    // Self {
    //   name,
    //   user_id,
    //   user_name,
    //   uptime_clock,
    //   device_access_regulation: DeviceAccessRegulation::new(),
    //   screen_access_regulation: ScreenAccessRegulation::new(),
    //   internet_access_regulation: InternetAccessRegulation::new(),
    // }

    todo!()
  } 
  
  pub fn construct(
    name: UserProfileName,
    user_id: UserId,
    user_name: UserName,
    uptime_clock: UserUptimeClock,
    device_access_regulation: DeviceAccessRegulation,
    screen_access_regulation: ScreenAccessRegulation,
    internet_access_regulation: InternetAccessRegulation,
  ) -> Self {
    // Self {
    //   name,
    //   user_id,
    //   user_name,
    //   uptime_clock,
    //   device_access_regulation,
    //   screen_access_regulation,
    //   internet_access_regulation,
    // }

    todo!()
  } 

  pub fn is_session_open_blocked(
    &self, 
    time: Time,
    instant: Instant,
  ) -> bool {
    self.screen_access_regulation.always_rules.are_some_active(instant)
    ||
    self.screen_access_regulation.time_range_rules.are_some_active(time, instant)
  }

  pub fn on_user_session_opened(&self) {

  }

  pub fn on_user_session_closed(&self) {

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
  user_profiles: HashMap<UuidV4, UserProfile>,
  user_names_to_profile_ids: HashMap<UserName, UuidV4>,
}

impl UserProfiles {
  pub fn new() -> Self {
    Self {
      user_profiles: HashMap::new(),
      user_names_to_profile_ids: HashMap::new(),
    }
  }

  pub fn construct(user_profiles: HashMap<UuidV4, UserProfile>) -> Self {
    let mut user_names_to_profile_ids = HashMap::new();
    for (key, value) in &user_profiles {
      user_names_to_profile_ids.insert(value.user_name.clone(), key.clone());
    }

    Self {
      user_profiles,
      user_names_to_profile_ids,
    }
  }

  pub fn get_profile_given_id(&self, user_profile_id: &UuidV4) -> Option<&UserProfile> {
    self.user_profiles.get(user_profile_id)
  }

  pub fn get_profile_given_id_mut(&mut self, user_profile_id: &UuidV4) -> Option<&mut UserProfile> {
    self.user_profiles.get_mut(user_profile_id)
  }

  pub fn get_profile_given_user_name(&self, user_name: &UserName) -> Option<&UserProfile> {
    let profile_id = self.user_names_to_profile_ids.get(user_name)?;

    self.user_profiles.get(profile_id)
  }

  pub fn get_users_number(&self) -> usize {
    self.user_profiles.len()
  }

  pub fn add_user(&mut self, user_id: UuidV4, user: UserProfile) {
    self.user_profiles.insert(user_id, user);
  }

  pub fn delete_user(&mut self, user_id: &UuidV4) {
    self.user_profiles.remove(user_id);
  }

  pub fn contains_user(&self, user_id: &UuidV4) -> bool {
    self.user_profiles.contains_key(user_id)
  }
}
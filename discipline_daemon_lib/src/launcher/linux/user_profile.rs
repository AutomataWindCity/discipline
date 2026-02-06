use serde::{Serialize, Deserialize};
use super::{UserProfileName, UserId, UserName, DeviceAccessRegulation, ScreenAccessRegulation, InternetAccessRegulation};

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
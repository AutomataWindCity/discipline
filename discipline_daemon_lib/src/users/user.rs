use serde::{Serialize, Deserialize};
use crate::x::{operating_system, regulation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub regulation_info: regulation::PerUserInfo,
  pub operating_system_info: operating_system::PerUserInfo,
}

impl User {
  pub fn new(
    operating_system_info: operating_system::PerUserInfo,
  ) -> Self {
    Self {
      regulation_info: regulation::PerUserInfo::new(),
      operating_system_info,
    }
  } 
}
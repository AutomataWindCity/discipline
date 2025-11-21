use serde::{Serialize, Deserialize};
use crate::x::{operating_system, regulation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedUser {
  pub regulation: regulation::PerUserInfo,
  operating_system_info: operating_system::PerUserInfo,
}

impl CachedUser {
  pub fn new(
    operating_system_info: operating_system::PerUserInfo,
  ) -> Self {
    Self {
      regulation: regulation::PerUserInfo::new(),
      operating_system_info,
    }
  } 
}
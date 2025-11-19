use serde::{Serialize, Deserialize};
use crate::x::{UserName, operating_system, regulation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  name: UserName,
  pub regulation: regulation::PerUserInfo,
  operating_system_info: operating_system::PerUserInfo,
}

impl User {
  pub fn new(
    name: UserName, 
    operating_system_info: operating_system::PerUserInfo,
  ) -> Self {
    Self {
      name,
      regulation: regulation::PerUserInfo::new(),
      operating_system_info,
    }
  } 
}
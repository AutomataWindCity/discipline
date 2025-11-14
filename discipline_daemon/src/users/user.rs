use serde::{Serialize, Deserialize};
use crate::x::{UserName, operating_system, regulation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  name: UserName,
  pub regulation: regulation::PerUserInfo,
  operating_system: operating_system::PerUserInfo,
}
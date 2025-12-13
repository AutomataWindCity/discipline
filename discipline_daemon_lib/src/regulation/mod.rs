pub mod block_device_access;
pub mod block_info_access;
pub mod block_account_access;
pub mod block_internet_access;
// pub mod database;

use serde::{Deserialize, Serialize};
use crate::x::MonotonicInstant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerUserInfo {
  pub block_device_access: block_device_access::Regulation,
  pub block_account_access: block_account_access::Regulation,
  pub block_internet_access: block_internet_access::Regulation,
}

impl PerUserInfo {
  pub fn new() -> Self {
    Self {
      block_device_access: block_device_access::Regulation::new(),
      block_account_access: block_account_access::Regulation::new(),
      block_internet_access: block_internet_access::Regulation::new(),
    }
  }
  
  pub fn construct(
    block_device_access: block_device_access::Regulation,
    block_account_access: block_account_access::Regulation,
    block_internet_access: block_internet_access::Regulation,
  ) -> Self {
    Self {
      block_account_access,
      block_device_access,
      block_internet_access,
    }
  }

  pub fn has_enabled_rules(&self, now: MonotonicInstant) -> bool {
    self.block_device_access.rules().are_some_rules_enabled(now)
    ||
    self.block_account_access.rules().are_some_rules_enabled(now)
    ||
    self.block_internet_access.rules().are_some_rules_enabled(now)
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossUserInfo {

}
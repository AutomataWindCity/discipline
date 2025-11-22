pub mod block_device_access;
pub mod block_info_access;
pub mod block_internet_access;
pub mod block_user_access;

use serde::{Deserialize, Serialize};
use crate::x::MonotonicInstant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerUserInfo {
  pub block_device_access: block_device_access::Regulation,
}

impl PerUserInfo {
  pub fn new() -> Self {
    Self {
      block_device_access: block_device_access::Regulation::new(),
    }
  }

  pub fn has_enabled_rules(&self, now: MonotonicInstant) -> bool {
    // self
    //   .block_device_access
    //   .rules
    //   .rules
    //   .values()
    //   .any(|it| it.is_enabled(now))
    todo!()
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossUserInfo {

}
pub mod block_device_access;
pub mod block_info_access;
pub mod block_internet_access;
pub mod block_user_access;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerUserInfo {
  pub block_device_access: block_device_access::BlockDeviceAccess,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossUserInfo {

}
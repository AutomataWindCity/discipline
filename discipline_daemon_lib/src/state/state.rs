use std::sync::Arc;
use tokio::sync::RwLock;
use crate::x::{UserGroup, MonotonicClock, rules, block_info_access};

pub struct State {
  pub clock: Arc<RwLock<MonotonicClock>>,
  pub users: Arc<RwLock<UserGroup>>,
  pub rules: Arc<RwLock<rules::CrossGroupInfo>>,
  pub block_info_access: Arc<RwLock<block_info_access::CrossVaultGroupInfo>>,
}
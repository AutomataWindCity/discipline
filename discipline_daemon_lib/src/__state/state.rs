use std::sync::Arc;
use tokio::sync::RwLock;
use crate::x::{UserGroup, MonotonicClock, users, rules, block_info_access};

pub struct State {
  pub clock: Arc<RwLock<MonotonicClock>>,
  pub users: Arc<RwLock<UserGroup>>,
  pub users_singleton: Arc<RwLock<users::UsersSingleton>>,
  pub rules_singleton: Arc<RwLock<rules::RulesSingleton>>,
  pub vaults_singleton: Arc<RwLock<block_info_access::VaultsSingleton>>,
}
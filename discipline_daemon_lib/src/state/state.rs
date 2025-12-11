use std::sync::Arc;
use tokio::sync::RwLock;
use crate::x::{UserGroup, MonotonicClock, rules};

pub struct State {
  pub clock: Arc<RwLock<MonotonicClock>>,
  pub users: Arc<RwLock<UserGroup>>,
  pub rules: Arc<RwLock<rules::CrossGroupInfo>>,
}
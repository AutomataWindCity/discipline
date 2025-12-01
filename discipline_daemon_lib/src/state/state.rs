use std::sync::Arc;
use tokio::sync::Mutex;
use crate::x::{UserGroup, monotonic::MonotonicClock, rules};

pub struct State {
  pub clock: MonotonicClock,
  pub users: UserGroup,
  pub rules: Arc<Mutex<rules::CrossGroupInfo>>,
}
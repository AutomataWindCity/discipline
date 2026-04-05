use crate::x::{MonotonicClock, RulesStats};
use super::UserProfiles;

pub struct State {
  pub user_profiles: UserProfiles,
  pub monotonic_clock: MonotonicClock,
  pub rules_stats: RulesStats,
}
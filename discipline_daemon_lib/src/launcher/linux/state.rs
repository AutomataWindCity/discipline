use crate::x::UuidV4;
use super::UserProfile;

pub struct State {
  user_profiles: HashMap<UuidV4, UserProfile>,
}
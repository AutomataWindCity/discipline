use std::collections::HashMap;
use crate::x::{User, UuidV4};

pub struct UserProfilesSingleton {
  maximum_user_number: usize,
}

impl Default for UserProfilesSingleton {
  fn default() -> Self {
    Self {
      maximum_user_number: 50,
    }
  }
}

impl UserProfilesSingleton {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      maximum_user_number,
    }
  }

  pub fn construct(maximum_user_number: usize) -> Self {
    Self {
      maximum_user_number,
    }
  }

  pub fn get_maximum_user_number(&self) -> usize {
    self.maximum_user_number
  }
}

#[derive(Debug)]
pub struct UserProfiles {
  user_profiles: HashMap<UuidV4, User>,
}

impl UserProfiles {
  pub fn new() -> Self {
    Self {
      user_profiles: HashMap::new(),
    }
  }

  pub fn construct(user_profiles: HashMap<UuidV4, User>) -> Self {
    Self {
      user_profiles,
    }
  }

  pub fn get_user(&self, user_id: &UuidV4) -> Option<&User> {
    self.user_profiles.get(user_id)
  }

  pub fn get_users_number(&self) -> usize {
    self.user_profiles.len()
  }

  pub fn add_user(&mut self, user_id: UuidV4, user: User) {
    self.user_profiles.insert(user_id, user);
  }

  pub fn delete_user(&mut self, user_id: &UuidV4) {
    self.user_profiles.remove(user_id);
  }

  pub fn contains_user(&self, user_id: &UuidV4) -> bool {
    self.user_profiles.contains_key(user_id)
  }
}
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use crate::{operating_system, x::{User, UuidV4}};

pub struct UsersSingleton {
  maximum_user_number: usize,
}

impl Default for UsersSingleton {
  fn default() -> Self {
    Self {
      maximum_user_number: 50,
    }
  }
}

impl UsersSingleton {
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
pub struct UserGroup {
  users_by_ids: HashMap<UuidV4, Arc<RwLock<User>>>,
}

impl UserGroup {
  pub fn new() -> Self {
    Self {
      users_by_ids: HashMap::new(),
    }
  }

  pub fn construct(users: HashMap<UuidV4, Arc<RwLock<User>>>) -> Self {
    Self {
      users_by_ids: users,
    }
  }

  pub fn get_user(&self, user_id: &UuidV4) -> Option<&Arc<RwLock<User>>> {
    self.users_by_ids.get(user_id)
  }

  pub fn get_users_number(&self) -> usize {
    self.users_by_ids.len()
  }

  pub fn add_user(&mut self, user_id: UuidV4, user: User) {
    self.users_by_ids.insert(user_id, Arc::new(RwLock::const_new(user)));
  }

  pub fn delete_user(&mut self, user_id: &UuidV4) {
    self.users_by_ids.remove(user_id);
  }

  pub fn contains_user(&self, user_id: &UuidV4) -> bool {
    self.users_by_ids.contains_key(user_id)
  }

  pub fn get_user_by_operating_system_user_name(
    &self, 
    user_name: operating_system::UserNameRef<'_>,
  ) -> Option<&Arc<RwLock<User>>> {
    // self.users_by_ids.get(k)
    todo!()
  }
}
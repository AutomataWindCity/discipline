use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use crate::x::{CachedUser, UuidV4};

#[derive(Debug, Clone)]
pub struct UserGroup {
  pub users: HashMap<UuidV4, Arc<Mutex<CachedUser>>>,
  pub maximum_user_number: usize,
}

impl UserGroup {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      users: HashMap::new(),
      maximum_user_number,
    }
  }

  pub fn get_user(&self, user_id: &UuidV4) -> Option<&Arc<Mutex<CachedUser>>> {
    self.users.get(user_id)
  }

  pub fn get_user_mut(&mut self, user_id: &UuidV4) -> Option<&mut Arc<Mutex<CachedUser>>> {
    self.users.get_mut(user_id)
  }
}
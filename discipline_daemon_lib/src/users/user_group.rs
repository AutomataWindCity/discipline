use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use crate::x::{User, UuidV4};

#[derive(Debug)]
pub struct UserGroup {
  pub users: HashMap<UuidV4, Arc<RwLock<User>>>,
  pub maximum_user_number: usize,
}

impl UserGroup {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      users: HashMap::new(),
      maximum_user_number,
    }
  }

  pub fn get_user(&self, user_id: &UuidV4) -> Option<&Arc<RwLock<User>>> {
    self.users.get(user_id)
  }

  pub fn get_user_mut(&mut self, user_id: &UuidV4) -> Option<&mut Arc<RwLock<User>>> {
    self.users.get_mut(user_id)
  }
}
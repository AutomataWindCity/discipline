use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::x::{User, UuidV4};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
  users: HashMap<UuidV4, Arc<Mutex<User>>>,
  maximum_user_number: usize,
}

impl UserGroup {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      users: HashMap::new(),
      maximum_user_number,
    }
  }

  pub fn get_user_mut(&mut self, user_id: &UuidV4) -> Option<&mut User> {
    self.users.get_mut(user_id)
  }
}
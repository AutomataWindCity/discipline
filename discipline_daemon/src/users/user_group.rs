use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::x::{User, UuidV4};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
  users: HashMap<UuidV4, User>,
  maximum_user_number: usize,
}

impl UserGroup {
  pub fn new(maximum_user_number: usize) -> Self {
    Self {
      users: HashMap::new(),
      maximum_user_number,
    }
  }
}
use std::collections::HashSet;
use crate::x::{Database, UuidV4, conditionals};

pub struct Daemon {
  database: Database,
  users: HashSet<UuidV4, User>,
  // rule_groups
}

pub struct User {
  regulation: Regulation, 
}

pub struct Regulation {
  device_access_regulation: DeviceAccessRegulation,
  account_access_regulation: AccountAccessRegulation,
  internet_access_regulation: InternetAccessRegulation,
}

pub struct DeviceAccessRegulation {
  block_at: conditionals::Weekly,
  block_for: conditionals::Countdown,
}

pub struct AccountAccessRegulation {
  block_at: conditionals::Weekly,
  block_for: conditionals::Countdown,
}

pub struct InternetAccessRegulation {
  block_at: conditionals::Weekly,
  block_for: conditionals::Countdown,
}

pub struct Rule {
  
}
pub struct Rules {
  
}
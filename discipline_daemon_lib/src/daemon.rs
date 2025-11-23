use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use crate::x::{UserGroup, monotonic::MonotonicClock, rules};

pub struct State {
  pub clock: MonotonicClock,
  pub users: UserGroup,
  pub rules: Arc<Mutex<rules::CrossGroupInfo>>,
}

pub struct Database {
  pub connection: crate::x::database::Connection,
  pub user_device_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_account_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_internet_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
}


pub struct DaemonLaunchConfiguration {
  procedure_call_server_port: u64,
  database_directory_path: PathBuf,
}

pub struct Daemon {
  launch_configuration: DaemonLaunchConfiguration,
  pub state: State,
  pub database: Database,
}
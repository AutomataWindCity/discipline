use std::path::PathBuf;

use crate::x::{rules, UserGroup};

pub struct State {
  pub users: UserGroup,
  pub rules: rules::CrossUserInfo,
}

pub struct DaemonLaunchConfiguration {
  rmote_procedure_call_server_port: u64,
  database_directory_path: PathBuf,
}

pub struct Daemon {
  launch_configuration: DaemonLaunchConfiguration,
  pub state: State,
}
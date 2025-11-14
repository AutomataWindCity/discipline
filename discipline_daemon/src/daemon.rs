use std::path::PathBuf;

use crate::x::UserGroup;

pub struct DaemonLaunchConfiguration {
  rmote_procedure_call_server_port: u64,
  database_directory_path: PathBuf,
}

pub struct Daemon {
  launch_configuration: DaemonLaunchConfiguration,
  pub users: UserGroup,
}
use std::path::PathBuf;
use crate::x::{Database, Server, State};

pub struct DaemonLaunchConfiguration {
  server_port: u64,
  database_directory: PathBuf,
}

pub struct Daemon {
  launch_configuration: DaemonLaunchConfiguration,
  pub state: State,
  pub database: Database,
  pub server: Server,
}

impl Daemon {
  pub fn open(configuration: DaemonLaunchConfiguration) {
    // let database = Database::
  }
}
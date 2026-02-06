use std::path::PathBuf;
use super::State;
use crate::x::{Database, Server};

pub struct DaemonLaunchConfiguration {
  pub api_server_port: u16,
  pub database_directory: PathBuf,
}

pub struct Daemon {
  pub state: State,
  pub database: Database,
  pub api_server: Server,
}
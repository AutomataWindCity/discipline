use std::path::PathBuf;

pub mod chronic;
pub mod other;
pub mod conditionals;
pub mod rules;
pub mod regulation;
pub mod operating_system;
pub mod users;
pub mod daemon;
pub mod database;
pub mod x;
pub mod server;
pub mod procedures;
pub mod state;
// pub mod vs;

#[tokio::main]
async fn main() {
  use crate::x::{Daemon, DaemonLaunchConfiguration};

  let daemon = Daemon::open(DaemonLaunchConfiguration {
    api_server_port: 9090,
    database_directory: PathBuf::from("/workspaces/discipline/discipline_daemon_lib/data"),
  }).await.unwrap();

  
}

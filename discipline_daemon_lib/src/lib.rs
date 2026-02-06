use std::path::PathBuf;

// mod ui_text;

mod rules_v2;

mod launcher;

pub mod chronic;
pub mod other;
pub use other::textual_error::*;
pub mod conditionals;
pub mod rules;
pub mod regulation;
pub mod operating_system;
pub mod users;
pub mod daemon;
pub mod database;
pub mod x;
pub mod protocol;
pub mod procedures;
pub mod state;
// pub mod vs;

#[tokio::main]
async fn main() {
  use crate::x::{Daemon, DaemonLaunchConfiguration};

  println!("Hi from main");
  let daemon = Daemon::open(DaemonLaunchConfiguration {
    api_server_port: 9090,
    database_directory: PathBuf::from("/workspaces/discipline/discipline_daemon_lib/data"),
  }).await.unwrap();

  daemon.start().await;
}

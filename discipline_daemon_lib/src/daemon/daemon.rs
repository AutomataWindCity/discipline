use std::{ops::Not, path::PathBuf};
use std::sync::Arc;
use crate::{operating_system, x::{Database, Server, State, TextualError, database}};

pub struct DaemonLaunchConfiguration {
  pub api_server_port: u16,
  pub database_directory: PathBuf,
}

pub struct Daemon {
  _configuration: DaemonLaunchConfiguration,
  pub state: State,
  pub database: Database,
  pub api_server: Server,
}

impl Daemon {
  pub async fn open(configuration: DaemonLaunchConfiguration) -> Result<Arc<Self>, TextualError> {
    // println!("Hi from open");
    let database = Database::open(configuration.database_directory.clone())
      .await
      .map_err(|error| {
        error.with_context("Opening Daemon")
      })?;

    let state = database::singleton::select_state(&database)
      .await
      .map_err(|error| {
        error.with_context("Opening Daemon")
      })?;

    let server = Server::new(configuration.api_server_port)
      .await
      .map_err(|error| {
        error.with_context("Opening Daemon")
      })?;

    Ok(Arc::new(Self {
      state,
      api_server: server,
      database,
      _configuration: configuration,
    }))
  }

  pub fn clone(self: &Arc<Self>) -> Arc<Self> {
    Arc::clone(self)
  }

  pub async fn start(self: Arc<Self>) {
    _ = self.clone().api_server.start_auto_serving(self).await;
  }

  pub async fn is_user_permitted_to_open_session(&self, user_name: operating_system::UserNameRef<'_>) -> bool {
    let user_group = self.state.users.read().await;
    let user = user_group.get_user_by_operating_system_user_name(user_name);
    let Some(user) = user else {
      return true;
    };

    let now = self.state.clock.read().await.now();
    let user = user.read().await;
    user
      .regulation_info
      .block_account_access
      .rules
      .are_some_rules_enabled(now)
      .not()
  }

  pub async fn on_user_session_closed(&self, user_name: operating_system::UserNameRef<'_>) {
    
  }

  pub async fn on_user_session_opened(&self, user_name: operating_system::UserNameRef<'_>) {
    
  }
}


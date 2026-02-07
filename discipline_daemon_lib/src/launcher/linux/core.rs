use std::path::PathBuf;
use crate::x::TextualErrorV2;
use super::{State, Database, Api};

pub struct LaunchConfiguration {
  pub api_server_port: u16,
  pub database_directory: PathBuf,
}

pub struct Daemon {
  database: Database,
  state: State,
  api: Api,
}

impl Daemon {
  pub fn open(
    configuration: LaunchConfiguration,
    textual_error: &mut impl TextualErrorV2,
  ) -> Result<Self, ()> {
    let mut textual_error_context = textual_error.context(
      "Launchnig Discipline Linux Daemon",
    );

    let database = Database::open(
      &mut textual_error_context, 
      configuration.database_directory,
    )?;

    let state = database.load_state(
      &mut textual_error_context,
    )?;

    let api = Api::create(
      &mut textual_error_context,
    )?;

    Ok(Self {
      api,
      state,
      database,
    })
  }

  // pub async fn start(self: Arc<Self>) {
  //   _ = self.clone().api_server.start_auto_serving(self).await;
  // }

  // pub async fn is_user_permitted_to_open_session(&self, user_name: operating_system::UserNameRef<'_>) -> bool {
  //   let user_group = self.state.users.read().await;
  //   let user = user_group.get_user_by_operating_system_user_name(user_name);
  //   let Some(user) = user else {
  //     return true;
  //   };

  //   let now = self.state.clock.read().await.now();
  //   let user = user.read().await;
  //   user
  //     .regulation_info
  //     .block_account_access
  //     .rules
  //     .are_some_rules_enabled(now)
  //     .not()
  // }
}
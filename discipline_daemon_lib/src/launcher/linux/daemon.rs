use std::path::PathBuf;
use crate::x::{DateTime, TextualErrorV2, Database};
use super::{State, Api, UserName, pam};

pub struct LaunchConfiguration {
  pub api_server_port: u16,
  pub database_directory: PathBuf,
  pub pam_server_path: PathBuf,
  pub pam_client_authentication_token: pam::AuthenticationToken,
}

pub struct Daemon {
  pub state: State,
  pub database: Database,
  pub api_server: Api,
  pub pam_server: pam::Server,
}

impl Daemon {
  pub fn open(
    configuration: LaunchConfiguration,
    textual_error: &mut impl TextualErrorV2,
  ) -> Result<Self, ()> {
    let mut textual_error_context = textual_error.context(
      "Launchnig Discipline Linux Daemon",
    );

    // let database = Database::open(
    //   &mut textual_error_context, 
    //   configuration.database_directory,
    // )?;

    // let state = database.load_state(
    //   &mut textual_error_context,
    // )?;

    // let api = Api::create(
    //   &mut textual_error_context,
    // )?;

    // let pam_server = pam::Server::new(
    //   configuration.pam_server_path, 
    //   configuration.pam_client_authentication_token, 
    //   textual_error,
    // )?;

    // Ok(Self {
    //   api_server: api,
    //   state,
    //   database,
    //   pam_server,
    // })
    todo!()
  }

  pub fn is_user_session_open_blocked(&self, user_name: &UserName) -> bool {
    self
      .state
      .user_profiles
      .get_profile_given_user_name(user_name)
      .map(|profile| {
        let time = DateTime::now().time();
        let instant = self.state.monotonic_clock.now();
        profile.is_session_open_blocked(time, instant)
      })
      .unwrap_or(false)
  }

  pub fn on_user_session_opened(&self, user_name: &UserName) {
    self
      .state
      .user_profiles
      .get_profile_given_user_name(user_name)
      .map(|profile| {
        profile.on_user_session_opened();
      });
  }

  pub fn on_user_session_closed(&self, user_name: &UserName) {
    self
      .state
      .user_profiles
      .get_profile_given_user_name(user_name)
      .map(|profile| {
        profile.on_user_session_closed();
      });
  }
}
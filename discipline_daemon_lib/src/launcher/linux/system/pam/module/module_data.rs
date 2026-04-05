use std::sync::{Mutex, MutexGuard};
use std::path::{Path, PathBuf};
use crate::x::{IsTextualError, OptionalTextualErrorContext};
// use super::{SystemLogger, ClientConnection, EstablishConnectionError, UserNameRef, ModuleConfiguration};
use super::{SystemLogger, UserNameRef, ModuleConfiguration};

struct ModuleData {
  logger: SystemLogger,
  configuration: ModuleConfiguration,
  // connection: ClientConnection,  
}

// TODO: Add a field containing magic bytes that we check
// when we get the Module from "pam_get_data" to verify that
// the data is our data.
pub struct ModuleDataMutex {
  mutex: Mutex<ModuleData>,
}

// discipline_installation_directory().join("linux_pam_module.log")
// discipline_installation_directory().join("linux_pam_module_configuration.json");

impl ModuleDataMutex {
  pub fn create(
    custom_log_file_path: PathBuf,
    configuration_file_path: impl AsRef<Path>,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, EstablishConnectionError> {
    let mut textual_error = textual_error
      .optional_context("Creating Discipline Linux-PAM Module Data");

    let logger = SystemLogger::create(custom_log_file_path);

    let configuration = ModuleConfiguration
      ::load(
        configuration_file_path, 
        &mut textual_error,
      )
      .map_err(|_| {
        EstablishConnectionError::Other
      })?;

    // TODO: Add more context to the textual error
    let connection = ClientConnection::connect(
      configuration.discipline_daemon_unix_domain_server_path.clone(), 
      &configuration.authentication_token,
      &mut textual_error,
    )?;

    Ok(Self {
      mutex: Mutex::new(ModuleData {
        logger,
        configuration,
        connection,
      }),
    })
  }

  fn lock(&self) -> Result<MutexGuard<'_, ModuleData>, ()> {
    self.mutex.lock().map_err(|_| ())
  }

  pub fn is_user_session_open_blocked(&self, user_name: UserNameRef<'_>) -> bool {
    let mut textual_error = OptionalTextualErrorContext::new("action");
    
    let Ok(mut data) = self.lock() else {
      return false;
    };

    data
      .connection
      .is_user_session_open_blocked(user_name, &mut textual_error)
      .unwrap_or(false)
  }

  pub fn on_session_opened(&self, user_name: UserNameRef<'_>) {
    let mut textual_error = OptionalTextualErrorContext::new("");

    let mut data = match self.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return;
      }
    };
    
    match data.connection.send_user_session_opened_notification(user_name, &mut textual_error) {
      Ok(value) => {
        value
      }
      Err(error) => {
        // TODO: Log the error.
        return;
      }
    };
  }

  pub fn on_session_closed(&self, user_name: UserNameRef<'_>) {
    let mut textual_error = OptionalTextualErrorContext::new("");

    let mut data = match self.mutex.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        // TODO: Log the error.
        return;
      }
    };

    if let Err(error) = data.connection.send_user_session_closed_notification(user_name, &mut textual_error) {
      // TODO: Log the error.
    }
  }
}
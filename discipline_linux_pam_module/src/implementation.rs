use std::net::SocketAddr;
use std::sync::Mutex;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use discipline_daemon::{TextualError, ToTextualError, TextualErrorContext};
use discipline_daemon::chronic::duration::Duration;
use discipline_daemon::operating_system::UserName;
use discipline_daemon::protocol::BlockingClientConnection;

use crate::discipline_installation_directory;

pub enum LoadModuleConfigurationError {
  ErrorWhileReadingConfigurationFile {
    error: std::io::Error,
    configuration_file_path: PathBuf,
  },
  ErrorWhileDeserializingFileContent {
    error: serde_json::Error,
    configuration_file_path: PathBuf,
    configuration_file_content: Vec<u8>,
  },
}

impl ToTextualError for LoadModuleConfigurationError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new("Loading the configuration for the Discipline Linux Pam Module from a json file");
    
    match self {
      Self::ErrorWhileReadingConfigurationFile { configuration_file_path, error: io_error } => {
        context.add_message("An io error occured while reading the configuration file");
        context.add_attachement_display("Configuration file path", configuration_file_path.display());
        context.add_attachement_display("Io error", io_error);
      }
      Self::ErrorWhileDeserializingFileContent { configuration_file_path, error, configuration_file_content: file_content } => {
        context.add_message("An error occured while deserializing the configuration file");
        context.add_attachement_display("Confiugration file path", configuration_file_path.display());
        context.add_attachement_display("Deserialization error", error);
        context.add_attachement_debug("Configuration file content", file_content);
      }
    }

    context
  }
}

pub enum CreateModuleError {
  ErrorWhileLoadingConfiguration(LoadModuleConfigurationError),
  ErrorWhileConnectingToDisciplineDaemon(TextualError)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfiguration {
  pam_call_timeout: Duration,
  pam_login_blocked_message: String,
  discipline_daemon_unix_domain_server_address: SocketAddr,
}

fn load_configuration(configuration_file_path: PathBuf) -> Result<ModuleConfiguration, LoadModuleConfigurationError> {
  let configuration_file_content = match std::fs::read(&configuration_file_path) {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(LoadModuleConfigurationError::ErrorWhileReadingConfigurationFile {
        error,
        configuration_file_path,
      });
    }
  };

  let configuration = match serde_json::from_slice(&configuration_file_content) {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(LoadModuleConfigurationError::ErrorWhileDeserializingFileContent { 
        error, 
        configuration_file_path,
        configuration_file_content,
      });
    }
  };

  Ok(configuration)
}

// TODO: Add a field containing magic bytes that we check
// when we get the Module from "pam_get_data" to verify that
// the data is our data.
pub struct Module {
  // discipline_installation_directory: PathBuf,
  // discipline_daemon_unix_server_path: PathBuf,
  // discipline_pam_configuration_path: PathBuf,
  discipline_daemon_connection: Mutex<BlockingClientConnection>,
}

impl Module {
  pub fn create() -> Result<Self, CreateModuleError> {
    let discipline_installation_directory = discipline_installation_directory();

    let discipline_pam_configuration_path = discipline_installation_directory.join("pam_module_configuration.json");

    // let discipline_daemon_unix_server_path = discipline_installation_directory.join("unix_domain_server");

    let configuration = load_configuration(discipline_pam_configuration_path).map_err(|error| {
      CreateModuleError::ErrorWhileLoadingConfiguration(error)
    })?;

    let discipline_daemon_connection = BlockingClientConnection
      ::connect(
        configuration.discipline_daemon_unix_domain_server_address,
        configuration.pam_call_timeout,
      )
      .map_err(|error| {
        CreateModuleError::ErrorWhileConnectingToDisciplineDaemon(error)
      })?;

    Ok(Self {
      // discipline_daemon_unix_server_path,
      // discipline_pam_configuration_path,
      discipline_daemon_connection: Mutex::new(discipline_daemon_connection),
    })
  }


  // Fatal error.
  //
  // TODO: log the error.
  //
  // Fallback to the safest course of action: Let the user
  // access their account. If we do otherwise, we might prevent
  // the user from accessing their account forever.

  pub fn is_login_blocked(&self, user_name: &UserName) -> Result<bool, ()> {
    let Ok(mut connection) = self.discipline_daemon_connection.lock() else {
      return Err(());
    };

    let Ok(is_login_blocked) = connection.is_user_account_access_blocked(user_name) else {
      return Err(());
    };

    Ok(is_login_blocked)
  }

  pub fn on_session_opened(&self, user_name: &UserName) {
    let mut connection = match self.discipline_daemon_connection.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        // TODO: Log the error.
        return;
      }
    };

    if let Err(error) = connection.notify_that_user_session_opened(user_name) {
      // TODO: Log the error.
    }
  }

  pub fn on_session_closed(&self, user_name: &UserName) {
    let mut connection = match self.discipline_daemon_connection.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        // TODO: Log the error.
        return;
      }
    };

    if let Err(error) = connection.notify_that_user_session_closed(user_name) {
      // TODO: Log the error.
    }
  }
}
use std::fmt::Debug;
use std::sync::Mutex;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use discipline_daemon::{TextualError, ToTextualError, TextualErrorContext};
use discipline_daemon::chronic::duration::Duration;
use discipline_daemon::operating_system::UserName;
use discipline_daemon::operating_system::linux_pam_api::ClientConnection;

use crate::*;

// #[derive(Debug)]
// pub enum LoadModuleConfigurationError {
//   ErrorWhileReadingConfigurationFile {
//     error: std::io::Error,
//     configuration_file_path: PathBuf,
//   },
//   ErrorWhileDeserializingFileContent {
//     error: serde_json::Error,
//     configuration_file_path: PathBuf,
//     configuration_file_content: Vec<u8>,
//   },
// }

// impl ToTextualError for LoadModuleConfigurationError {
//   fn to_textual_error_context(&self) -> TextualErrorContext {
//     let mut context = TextualErrorContext::new("Loading the configuration for the Discipline Linux Pam Module from a json file");
    
//     match self {
//       Self::ErrorWhileReadingConfigurationFile { configuration_file_path, error: io_error } => {
//         context.add_message("An io error occured while reading the configuration file");
//         context.add_attachement_display("Configuration file path", configuration_file_path.display());
//         context.add_attachement_display("Io error", io_error);
//       }
//       Self::ErrorWhileDeserializingFileContent { configuration_file_path, error, configuration_file_content: file_content } => {
//         context.add_message("An error occured while deserializing the configuration file");
//         context.add_attachement_display("Confiugration file path", configuration_file_path.display());
//         context.add_attachement_display("Deserialization error", error);
//         context.add_attachement_debug("Configuration file content", file_content);
//       }
//     }

//     context
//   }
// }

// #[derive(Debug)]
// pub enum CreateModuleError {
//   ErrorWhileLoadingConfiguration(LoadModuleConfigurationError),
//   ErrorWhileConnectingToDisciplineDaemon(TextualError)
// }

// impl CreateModuleError {
//   pub fn write_to_textual_error(&self, textual_error: &mut TextualError) {
//     textual_error.change_context("Creating the initial Discipline Linux Pam Module Data");

//     match self {
//       Self::ErrorWhileLoadingConfiguration(error) => {
//         textual_error.add_message("An error occured while loading the configuration");
//         textual_error.with_attachement_display("Err", value)
//       }
//     }
//   }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfiguration {
  pam_call_timeout: Duration,
  pam_login_blocked_message: String,
  discipline_daemon_unix_domain_server_path: PathBuf,
}

// fn load_configuration(configuration_file_path: PathBuf) -> Result<ModuleConfiguration, LoadModuleConfigurationError> {
//   let configuration_file_content = match std::fs::read(&configuration_file_path) {
//     Ok(value) => {
//       value
//     }
//     Err(error) => {
//       return Err(LoadModuleConfigurationError::ErrorWhileReadingConfigurationFile {
//         error,
//         configuration_file_path,
//       });
//     }
//   };

//   let configuration = match serde_json::from_slice(&configuration_file_content) {
//     Ok(value) => {
//       value
//     }
//     Err(error) => {
//       return Err(LoadModuleConfigurationError::ErrorWhileDeserializingFileContent { 
//         error, 
//         configuration_file_path,
//         configuration_file_content,
//       });
//     }
//   };

//   Ok(configuration)
// }

fn load_configuration_or_textual_error(configuration_file_path: PathBuf) -> Result<ModuleConfiguration, TextualError> {
  let configuration_file_content = std::fs::read(&configuration_file_path)
    .map_err(|error| {
      TextualError::new("Reading the json confiugration file for Discipline Linux-PAM Module")
        .with_message("A filesystem error occured while reading the file")
        .with_attachement_display("Filesystem error", error)
        .with_attachement_display("Configuration file path", configuration_file_path.display())
    })?;

  let configuration = serde_json::from_slice(&configuration_file_content)
    .map_err(|error| {
      TextualError::new("Reading the json configuration file for Discipline Linux-PAM Module")
        .with_message("An error occured while deserializing the file content")
        .with_attachement_display("Deserializing error", error)
        .with_attachement_display("Configuration file path", configuration_file_path.display())
        // .with_attachement_display("Configuration file content", configuration_file_content)
    })?;

  Ok(configuration)
}

// TODO: Add a field containing magic bytes that we check
// when we get the Module from "pam_get_data" to verify that
// the data is our data.
pub struct Module {
  // discipline_installation_directory: PathBuf,
  // discipline_daemon_unix_server_path: PathBuf,
  // discipline_pam_configuration_path: PathBuf,
  runtime: Runtime,
  discipline_daemon_connection: Mutex<ClientConnection>,
  logger: Logger
}

impl Module {
  pub async fn create() -> Result<Self, ()> {
    let logger = Logger::create(discipline_installation_directory().join("linux_pam_module.log"));

    let runtime = match Runtime::new() {
      Ok(value) => {
        value
      }
      Err(error) => {
        logger.write_displayable(
          TextualError::new("Creating Discipline Linux-PAM Module Data")
            .with_message("An error occured while creating a tokio async runtime.")
            .with_attachement_display("Tokio error", error)
        );
        return Err(())
      }
    };

    let discipline_pam_configuration_path = discipline_installation_directory()
      .join("linux_pam_module_configuration.json");

    let configuration = match load_configuration_or_textual_error(discipline_pam_configuration_path) {
      Ok(value) => {
        value
      }
      Err(error) => {
        logger.write_displayable(
          error
            .with_context("Creating Discipline Linux-PAM Module Data")
        );
        return Err(());
      }
    };

    let discipline_daemon_connection = ClientConnection::connect_or_textual_error(
      &configuration.discipline_daemon_unix_domain_server_path, 
      password
    );

    let discipline_daemon_connection = BlockingClientConnection
      ::connect(
        &configuration.discipline_daemon_unix_domain_server_path,
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

  pub fn is_login_blocked(&self, user_name: &UserName) -> bool {
    let Ok(mut connection) = self.discipline_daemon_connection.lock() else {
      return false;
    };

    let Ok(is_login_blocked) = connection.is_user_account_access_blocked(user_name) else {
      return false;
    };

    is_login_blocked
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
use std::fmt::Debug;
use std::sync::Mutex;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
// use tokio::sync::Mutex;

use crate::x::{TextualError, Duration};
use crate::x::operating_system::{UserName, UserNameRef};
use super::{Logger, ClientConnection};

use mio::{Events, Poll, Interest, Token};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfiguration {
  password: String,
  pam_call_timeout: Duration,
  pam_login_blocked_message: String,
  discipline_daemon_unix_domain_server_path: PathBuf,
}

fn load_configuration_or_textual_error(configuration_file_path: impl AsRef<Path>) -> Result<ModuleConfiguration, TextualError> {
  let configuration_file_content = std::fs::read(&configuration_file_path)
    .map_err(|error| {
      TextualError::new("Reading the json confiugration file for Discipline Linux-PAM Module")
        .with_message("A filesystem error occured while reading the file")
        .with_attachement_display("Filesystem error", error)
        .with_attachement_display("Configuration file path", configuration_file_path.as_ref().display())
    })?;

  let configuration = serde_json::from_slice(&configuration_file_content)
    .map_err(|error| {
      TextualError::new("Reading the json configuration file for Discipline Linux-PAM Module")
        .with_message("An error occured while deserializing the file content")
        .with_attachement_display("Deserializing error", error)
        .with_attachement_display("Configuration file path", configuration_file_path.as_ref().display())
        // .with_attachement_display("Configuration file content", configuration_file_content)
    })?;

  Ok(configuration)
}

pub struct ModuleData {
  logger: Logger,
  runtime: Runtime,
  connection: ClientConnection,
  
}

// TODO: Add a field containing magic bytes that we check
// when we get the Module from "pam_get_data" to verify that
// the data is our data.
pub struct ModuleDataMutex {
  logger: Logger,
  runtime: Runtime,
  discipline_daemon_connection: Mutex<ClientConnection>,
  mutex: Mutex<ModuleData>,
  connection: Mutex<ClientConnection>,
  poll: Poll,
  events: Events,
}


// discipline_installation_directory().join("linux_pam_module.log")
// discipline_installation_directory().join("linux_pam_module_configuration.json");

impl ModuleDataMutex {
  pub async fn create(
    custom_log_file_path: PathBuf,
    discipline_pam_module_configuration_file_path: impl AsRef<Path>,
  ) -> Result<Self, TextualError> {
    let logger = Logger::create(custom_log_file_path);

    let runtime = Runtime::new().map_err(|error| {
      TextualError::new("Creating Discipline Linux-PAM Module Data")
        .with_message("An error occured while creating a tokio async runtime.")
        .with_attachement_display("Tokio error", error)
    })?;

    let configuration = load_configuration_or_textual_error(discipline_pam_module_configuration_file_path).map_err(|error| {
      error.with_context("Creating Discipline Linux-PAM Module Data")
    })?;

    let discipline_daemon_connection = ClientConnection::connect_or_textual_error(
      &configuration.discipline_daemon_unix_domain_server_path, 
      &configuration.password,
    )
      .await
      .map_err(|error| {
        error.with_context("Creating Discipline Linux-PAM Module Data")
      })?;

    Ok(Self {
      logger,
      runtime,
      discipline_daemon_connection: Mutex::new(discipline_daemon_connection),
    })
  }

  
  pub async fn is_session_open_permitted(&self, user_name: UserNameRef<'_>) -> bool {
    let mut data = match self.mutex.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return false;
      }
    };

    let mut connection = match self.connection.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return false;
      }
    };

    let is_login_blocked = match data.runtime.block_on(connection.is_user_session_open_permitted_or_textual_error(user_name)) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return false;
      }
    };

    is_login_blocked
  }

  pub fn on_session_opened(&self, user_name: &UserName) {
    let data = match self.mutex.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return;
      }
    };

    let mut connection = match self.connection.lock() {
      Ok(value) => {
        value
      }
      Err(error) => {
        // TODO: Log the error.
        return;
      }
    };

    if let Err(error) = data.runtime.block_on(future)connection.notify_that_user_session_opened(user_name) {
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

  // Fatal error.
  //
  // TODO: log the error.
  //
  // Fallback to the safest course of action: Let the user
  // access their account. If we do otherwise, we might prevent
  // the user from accessing their account forever.

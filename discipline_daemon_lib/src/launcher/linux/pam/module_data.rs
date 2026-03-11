use std::fmt::Debug;
use std::sync::{Mutex, MutexGuard};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::x::{Duration, IsTextualError, OptionalTextualErrorContext};
use super::{Logger, ClientConnection, AuthenticationToken, EstablishConnectionError, UserName, UserNameRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfiguration {
  authentication_token: AuthenticationToken,
  pam_call_timeout: Duration,
  pam_login_blocked_message: String,
  discipline_daemon_unix_domain_server_path: PathBuf,
}

fn load_configuration(
  configuration_file_path: impl AsRef<Path>,
  textual_error: &mut impl IsTextualError,
) -> Result<ModuleConfiguration, ()> {
  let mut textual_error = textual_error.optional_context("Loading Discpline Linux-PAM Module Configuration from file");

  let configuration_file_content = match std::fs::read(&configuration_file_path) {
    Ok(value) => {
      value
    }
    Err(error) => {
      textual_error.add_message("A filesystem error occured");
      textual_error.add_attachement_display("Filesystem error", error);
      textual_error.add_attachement_display("Configuration file path", configuration_file_path.as_ref().display());
      return Err(());
    }
  };

  let configuration = match serde_json::from_slice(&configuration_file_content) {
    Ok(value) => {
      value
    }
    Err(error) => {
      textual_error.change_context("Deserializing the configuration file content, which is in JSON format");
      textual_error.add_message("Deserialization failed");
      textual_error.add_attachement_display("Deserializing error", error);
      textual_error.add_attachement_display("Configuration file path", configuration_file_path.as_ref().display());
      // TODO: Add a flag fo whether to log the file content, too, or not.
      return Err(());
    }
  };

  Ok(configuration)
}

pub struct ModuleData {
  logger: Logger,
  configuration: ModuleConfiguration,
  connection: ClientConnection,  
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
    let mut textual_error = textual_error.optional_context("Creating Discipline Linux-PAM Module Data");

    let logger = Logger::create(custom_log_file_path);

    let configuration = match load_configuration(configuration_file_path, &mut textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        return Err(EstablishConnectionError::Other);
      }
    };

    // TODO: Add more context to the textual error
    let connection = ClientConnection::connect(
      &configuration.discipline_daemon_unix_domain_server_path, 
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
    todo!()
  }

  pub fn is_user_session_open_blocked(&self, user_name: UserNameRef<'_>) -> bool {
    let mut data = match self.lock() {
      Ok(value) => {
        value
      }
      Err(()) => {
        return false;
      }
    };

    let mut textual_error = OptionalTextualErrorContext::new("action");

    let is_user_session_open_blocked = match data.connection.is_user_session_open_blocked(user_name, &mut textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        return false;
      }
    };

    is_user_session_open_blocked
  }

  pub fn on_session_opened(&self, user_name: UserNameRef<'_>) {
    let mut textual_error = OptionalTextualErrorContext::new("");

    let mut data = match self.mutex.lock() {
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

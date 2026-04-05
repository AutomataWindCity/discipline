use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
use crate::x::{Duration, IsTextualError};
use super::{AuthenticationToken};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfiguration {
  pub authentication_token: AuthenticationToken,
  pub pam_call_timeout: Duration,
  pub pam_login_blocked_message: String,
  pub discipline_daemon_unix_domain_server_path: PathBuf,
}

impl ModuleConfiguration {
  pub fn load(
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
        // TODO: Add a flag to specify whether to log the file content, too, or not.
        return Err(());
      }
    };
  
    Ok(configuration)
  }

}


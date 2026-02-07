use std::{path::Path, sync::Arc};
use tokio::sync::Semaphore;
use tokio::{net::UnixListener, spawn};
use crate::x::protocol::x::DatagramConnection;
use crate::x::Daemon;
use super::*;

// pub struct Stream {
//   stream: mio::
// }

// TODO: 
pub const MAXIMUM_MESSAGE_LENGTH: usize = 0;
// TODO:
pub const CLIENT_PASSWORD: String = String::new();

pub struct Server {
  listener: UnixListener,
  semaphore: Arc<Semaphore>,
}

impl Server {
  pub async fn new(path: impl AsRef<Path>) -> Result<Self, TextualError> {
    UnixListener::bind(path)
      .map_err(|error| {
        TextualError::new("Creating Discipline Daemon Server for Discipline Linux-PAM Module")
          .with_attachement_display("Io error", error)
      })
      .map(|listener| {
        Self { 
          listener,
          semaphore: Arc::new(Semaphore::const_new(1)),
        }
      })
  }

  pub async fn start_auto_processing(&mut self, daemon: Arc<Daemon>) {
    loop {
      let semaphore = Arc::clone(&self.semaphore);

      let permit = match semaphore.acquire_owned().await {
        Ok(value) => {
          value
        }
        Err(error) => {
          let mut textaul_error = TextualError::new("Discipline linux-PAM Module Server");
          textaul_error.add_message("An error occured while aquiring a permit for a new connection");
          textaul_error.add_attachement_display("Error", error);
          eprintln!("{textaul_error}");
          return;
        }
      };

      let connection = match self.listener.accept().await {
        Ok(value) => {
          value.0
        }
        Err(error) => {
          let mut textaul_error = TextualError::new("Discipline linux-PAM Module Server");
          textaul_error.add_message("An error occured while accepting a new connection");
          textaul_error.add_attachement_display("Error", error);
          eprintln!("{textaul_error}");
          continue;
        }
      };

      let connection = DatagramConnection::construct(
        connection, 
        MAXIMUM_MESSAGE_LENGTH,
      );

      let Ok(mut connection) = ServerConnection::establish(connection).await else {
        continue;
      };

      let daemon = Arc::clone(&daemon);
      spawn(async move {
        connection.start_auto_processing(daemon).await;
        drop(permit);
      });
    }
  }
}

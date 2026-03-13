use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::net::UnixListener;
use tokio::spawn;
use crate::{IsTextualError, OptionalTextualErrorContext};
use super::*;

pub struct Server {
  listener: UnixListener,
  semaphore: Arc<Semaphore>,
  authentication_token: AuthenticationToken,
  maximum_message_length: usize,
  maximum_concurrent_connections: usize,
}

impl Server {
  pub async fn new(
    path: impl AsRef<Path>,
    authentication_token: AuthenticationToken,
    maximum_message_length: usize,
    maximum_concurrent_connections: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error.optional_context("Creating Discipline Linux-PAM Module Server");

    let listener = match UnixListener::bind(path) {
      Ok(value) => {
        value
      }
      Err(error) => {
        textual_error.add_message("An io error occured while binding the UnixListener");
        textual_error.add_attachement_display("Io error", error);
        return Err(());
      }
    };
      
    Ok(Self { 
      listener,
      semaphore: Arc::new(Semaphore::const_new(maximum_concurrent_connections)),
      authentication_token,
      maximum_message_length,
      maximum_concurrent_connections,
    })
  }

  pub async fn start_auto_processing(&mut self, daemon: Arc<Daemon>) {
    loop {
      let mut textual_error = OptionalTextualErrorContext::new("Discipline linux-PAM Module Server processing incoming connections");

      let semaphore = Arc::clone(&self.semaphore);

      let permit = match semaphore.acquire_owned().await {
        Ok(value) => {
          value
        }
        Err(error) => {
          let mut textual_error = textual_error.optional_context("Aquiring a permit from the server's semaphore");
          textual_error.add_attachement_display("Error", error);
          // TODOL Use a proper loggin mechanism.
          eprintln!("{textual_error}");
          
          // Aquiring a permit can only fail if the semaphore is closed, 
          // which shouldn't do before we close the server. So, this is
          // a fatal error.
          return;
        }
      };

      let connection = match self.listener.accept().await {
        Ok(value) => {
          value.0
        }
        Err(error) => {
          let mut textaul_error = textual_error.optional_context("Accepting a new connection");
          textaul_error.add_message("An io error occured");
          textaul_error.add_attachement_display("Io error", error);

          // TODO: Use a proper logging mechanism.
          eprintln!("{textaul_error}");

          // Since this isn't a fatal error, continue processing incoming connections.
          continue;
        }
      };

      let stream = ServerStream::construct(AsyncStream::construct(
        connection, 
        self.maximum_message_length,
      ));

      let Ok(mut connection) = ServerConnection::establish(stream, &self.authentication_token, &mut textual_error).await else {
        continue;
      };

      let daemon = Arc::clone(&daemon);
      // TODO: This will panic because the async runtime isn't configured globally. FIX.
      spawn(async move {
        connection.start_auto_processing(daemon).await;
        drop(permit);
      });
    }
  }
}

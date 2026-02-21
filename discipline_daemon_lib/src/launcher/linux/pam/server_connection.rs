use std::sync::Arc;
use crate::x::Daemon;
use crate::x::protocol::x::DatagramConnection;
use super::*;

pub struct ServerConnection {
  connection: DatagramConnection,
}

impl ServerConnection {
  pub async fn establish(mut connection: DatagramConnection) -> Result<Self, TextualError> {
    let message: EstablishConnection = connection
      .recv_or_textual_error()
      .await
      .map_err(|error| {
        TextualError::new("Discipline Linux-PAM Module Server establishing a connection with client")
          .with_message("An io error occured while reading the EstablishConnection message")
          .with_attachement_display("Io error", error)
      })?;

    if message.password != CLIENT_PASSWORD {
      // if let Err(error) = connection
      //   .send(&EstablishConnectionReply::IncorrectPassword)
      //   .await 
      // {
      //   textual_error.with_message("Failed to send an EstablishConnectionReply message to client due to")
      // }

      return Err(
        TextualError::new("Discipline Linux-PAM Module Server establishing a connection with client")
          .with_message("The client's password was incorrect")
      );
    }

    connection
      .send_or_textual_error(&EstablishConnectionReply::ConnectionEstablished)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module Server establishing a connection with client")
          .with_message("Failed to send EstablishConnectionReply::ConnectionEstablished")
      })?;

    Ok(Self { 
      connection,
    })
  }

  pub async fn start_auto_processing(&mut self, daemon: Arc<Daemon>) {
    loop {
      let client_message: ClientMessage = match self.connection.recv_or_textual_error().await {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!(
            "{:?}", 
            error
              .with_context("Discipline Daemon Linux-PAM Server Connection processing incoming messages")
              .with_message("An error occured while receiving a message from Discipline Linux-PAM Module")
          );
          return;
        }
      };

      match client_message {
        ClientMessage::IsUserSessionOpenPermitted(message) => {
          let message = IsUserSessionOpenPermittedReply { 
            is_user_session_open_permitted: daemon.is_user_permitted_to_open_session(message.user_name.as_ref()).await,
          };
          
          if let Err(mut error) = self
            .connection
            .send_or_textual_error(&message)
            .await
          {
            error.change_context("Discipline Daemon Linux-PAM Server Connection processing incoming messages ");
            error.add_message("Failed to send IsUserSessionOpenPermittedReply");
            eprintln!("{error:?}");
          }
        }
        ClientMessage::UserSessionOpenedNotification(notification) => {
          daemon.on_user_session_opened(notification.user_name.as_ref()).await;
        }
        ClientMessage::UserSessionClosedNotification(notification) => {
          daemon.on_user_session_closed(notification.user_name.as_ref()).await;
        }
      }
    }
  }
}

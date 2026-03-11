use std::sync::Arc;
use crate::{IsTextualError, OptionalTextualErrorContext};
use crate::x::Daemon;
use super::*;

pub struct ServerStream {
  stream: Stream,
}

impl ServerStream {
  pub fn construct(stream: Stream) -> Self {
    Self { stream }
  }

  pub fn recv_establish_connection(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<EstablishConnection, ()> {
    self.stream.recv(textual_error)
  }

  pub fn send_establish_connection_reply(
    &mut self,
    reply: EstablishConnectionReply,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(&reply, textual_error)
  }

  pub fn recv_client_message(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<ClientMessage, ()> {
    self.stream.recv(textual_error)
  }

  pub fn send_client_message(
    &mut self,
    client_message: &ClientMessage,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(client_message, textual_error)
  }
}

pub struct ServerConnection {
  stream: ServerStream,
}

impl ServerConnection {
  pub async fn establish(
    mut stream: ServerStream,
    password: &String,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error.optional_context("Discipline Linux-PAM Module Server establishing a connection with client");

    let establish_connection = match stream.recv_establish_connection(&mut textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        textual_error.add_message("An io error occured while reading the EstablishConnection message");
        return Err(());
      }
    };

    if establish_connection.password != *password {
      textual_error.add_message("The client's password was incorrect");

      let reply = EstablishConnectionReply::IncorrectPassword;
      let mut textual_error = textual_error.optional_context("Sending EstablishConnectionReply::IncorrectPassword message to client");
      if let Err(()) = stream.send_establish_connection_reply(reply, &mut textual_error) {
        textual_error.add_message("Failed to send an ")
      }

      return Err(());
    }

    let reply = EstablishConnectionReply::ConnectionEstablished;
    let mut textual_error = textual_error.optional_context("Sending EstablishConnectionReply::ConnectionEstablished");
    if let Err(()) = stream.send_establish_connection_reply(reply, &mut textual_error) {
      return Err(());
    }

    Ok(Self { 
      stream,
    })
    // TextualError::new("Discipline Linux-PAM Module Server establishing a connection with client")

    // let message: EstablishConnection = connection
    //   .recv_or_textual_error()
    //   .await
    //   .map_err(|error| {
    //   })?;

    // if message.password != CLIENT_PASSWORD {
    //   // if let Err(error) = connection
    //   //   .send(&EstablishConnectionReply::IncorrectPassword)
    //   //   .await 
    //   // {
    //   //   textual_error.with_message("Failed to send an EstablishConnectionReply message to client due to")
    //   // }

    //   return Err(
    //     TextualError::new("Discipline Linux-PAM Module Server establishing a connection with client")
    //       .with_message("The client's password was incorrect")
    //   );
    // }

    // connection
    //   .send_or_textual_error(&EstablishConnectionReply::ConnectionEstablished)
    //   .await
    //   .map_err(|error| {
    //     error
    //       .with_context("Discipline Linux-PAM Module Server establishing a connection with client")
    //       .with_message("Failed to send EstablishConnectionReply::ConnectionEstablished")
    //   })?;
  }

  pub async fn start_auto_processing(&mut self, daemon: Arc<Daemon>) {
    loop {
      let mut textual_error = OptionalTextualErrorContext::new("Discipline Daemon Linux-PAM Server Connection processing incoming messages");

      let client_message: ClientMessage = match self.stream.recv_client_message(&mut textual_error) {
        Ok(value) => {
          value
        }
        Err(()) => {
          // TODO: log via a proper logging mechanism
          eprintln!("{}", textual_error);
          return;
        }
      };

      match client_message {
        ClientMessage::IsUserSessionOpenPermitted(message) => {
          let message = IsUserSessionOpenPermittedReply { 
            is_user_session_open_permitted: daemon.is_user_permitted_to_open_session(message.user_name.as_ref()).await,
          };
          
          if let Err(mut error) = self
            .stream
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

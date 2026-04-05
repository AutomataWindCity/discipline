use std::sync::Arc;
use crate::{IsTextualError, OptionalTextualErrorContext};
use super::*;

pub struct ServerStream {
  stream: AsyncStream,
}

impl ServerStream {
  pub fn construct(stream: AsyncStream) -> Self {
    Self { stream }
  }

  pub async fn read_establish_connection(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<EstablishConnection, ()> {
    self.stream.read(
      &BincodeSerializationFormat,
      textual_error,
    ).await
  }

  pub async fn write_establish_connection_reply(
    &mut self,
    reply: EstablishConnectionReply,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.write(
      &reply, 
      &BincodeSerializationFormat,
      textual_error,
    ).await
  }

  pub async fn read_client_message(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<ClientMessage, ()> {
    self.stream.read(
      &BincodeSerializationFormat,
      textual_error,
    ).await
  }

  // pub fn write_client_message(
  //   &mut self,
  //   client_message: &ClientMessage,
  //   textual_error: &mut impl IsTextualError,
  // ) -> Result<(), ()> {
  //   self.stream.write(client_message, textual_error)
  // }

  pub async fn write_is_user_session_open_blocked_reply(
    &mut self,
    client_message: &IsUserSessionOpenBlockedReply,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.write(
      client_message, 
      &BincodeSerializationFormat,
      textual_error,
    ).await
  }
}

pub struct ServerConnection {
  stream: ServerStream,
}

impl ServerConnection {
  pub async fn establish(
    mut stream: ServerStream,
    authentication_token: &AuthenticationToken,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error
      .optional_context("Discipline Linux-PAM Module Server establishing a connection with client");

    let establish_connection = match stream.read_establish_connection(&mut textual_error).await {
      Ok(value) => {
        value
      }
      Err(()) => {
        textual_error.add_message("An io error occured while reading the EstablishConnection message");
        return Err(());
      }
    };

    if establish_connection.authentication_token != *authentication_token {
      textual_error.add_message("The client's password was incorrect");

      let reply = EstablishConnectionReply::UnrecognizedAuthenticationToken;
      let mut textual_error = textual_error.optional_context("Sending EstablishConnectionReply::IncorrectPassword message to client");
      if let Err(()) = stream.write_establish_connection_reply(reply, &mut textual_error).await {
        textual_error.add_message("Failed to send an ")
      }

      return Err(());
    }

    let reply = EstablishConnectionReply::ConnectionEstablished;
    let mut textual_error = textual_error.optional_context("Sending EstablishConnectionReply::ConnectionEstablished");
    if let Err(()) = stream.write_establish_connection_reply(reply, &mut textual_error).await {
      return Err(());
    }

    Ok(Self { 
      stream,
    })
    // TextualError::new("Discipline Linux-PAM Module Server establishing a connection with client")

    // let message: EstablishConnection = connection
    //   .read_or_textual_error()
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
    //   .write_or_textual_error(&EstablishConnectionReply::ConnectionEstablished)
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

      let client_message: ClientMessage = match self.stream.read_client_message(&mut textual_error).await {
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
        ClientMessage::IsUserSessionOpenBlocked(message) => {
          let is_user_session_open_blocked = daemon.is_user_session_open_blocked(message.user_name.as_ref());

          let message = IsUserSessionOpenBlockedReply { 
            is_user_session_open_blocked,
          };
          
          if let Err(()) = self
            .stream
            .write_is_user_session_open_blocked_reply(&message, &mut textual_error)
            .await
          {
            eprintln!("{textual_error}");
            // TODO: Send connection closed.
            return;
          }
        }
        ClientMessage::UserSessionOpenedNotification(notification) => {
          daemon.on_user_session_opened(notification.user_name.as_ref());
        }
        ClientMessage::UserSessionClosedNotification(notification) => {
          daemon.on_user_session_closed(notification.user_name.as_ref());
        }
      }
    }
  }
}

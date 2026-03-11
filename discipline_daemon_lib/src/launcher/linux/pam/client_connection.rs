use std::path::PathBuf;
use crate::IsTextualError;
use super::super::UserNameRef;
use super::{Stream, EstablishConnectionRef, EstablishConnectionReply, EstablishConnectionError, IsUserSessionOpenPermittedRef, IsUserSessionOpenPermittedReply, UserSessionClosedNotificationRef, UserSessionOpenedNotificationRef};

pub struct ClientStream {
  stream: Stream,
}

impl ClientStream {
  pub fn construct(stream: Stream) -> Self {
    Self { stream }
  }

  pub fn send_establish_connection(
    &mut self, 
    password: &str, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(
      &EstablishConnectionRef {
        password,
      }, 
      textual_error,
    )
  }

  pub fn recv_establish_connection_repl(
    &mut self,
    textual_error: &mut impl IsTextualError
  ) -> Result<EstablishConnectionReply, ()> {
    self.stream.recv(textual_error)
  }

  pub fn is_user_session_open_permitted(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<bool, ()> {
    self.stream.send(
      &IsUserSessionOpenPermittedRef {
        user_name,
      },
      textual_error,
    )?;

    let reply: IsUserSessionOpenPermittedReply = self.stream.recv(textual_error)?;

    Ok(reply.is_user_session_open_permitted)
  }

  pub fn send_user_session_opened_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(
      &UserSessionOpenedNotificationRef {
        user_name,
      },
      textual_error,
    )
  }

  pub fn send_user_session_closed_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(
      &UserSessionClosedNotificationRef {
        user_name,
      },
      textual_error,
    )
  }
}

// TODO
const MAXIMUM_MESSAGE_CONTENT_LENGTH: usize = 876868686876;

pub struct ClientConnection {
  stream: ClientStream,
}

impl ClientConnection {
  pub fn connect(
    path: PathBuf,
    password: &str, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, EstablishConnectionError> {
    let stream = match Stream::connect(path, MAXIMUM_MESSAGE_CONTENT_LENGTH, textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        return Err(EstablishConnectionError::Other);
      }
    };

    let mut stream = ClientStream::construct(stream);
    
    if let Err(()) = stream.send_establish_connection(password, textual_error) {
      return Err(EstablishConnectionError::Other);
    }

    let reply = match stream.recv_establish_connection_repl(textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        return Err(EstablishConnectionError::Other);
      }
    };

    match reply {
      EstablishConnectionReply::ServerBusy => {
        Err(EstablishConnectionError::ServerBusy)
      }
      EstablishConnectionReply::IncorrectPassword => {
        Err(EstablishConnectionError::IncorrectPassword)
      }
      EstablishConnectionReply::ConnectionEstablished => {
        Ok(ClientConnection { stream })
      }
    }
  }
}
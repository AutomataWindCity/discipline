use std::path::PathBuf;

use crate::IsTextualError;
use super::super::UserNameRef;
use super::{Stream, EstablishConnectionRef, EstablishConnectionError, IsUserSessionOpenPermittedRef, IsUserSessionOpenPermittedReply, UserSessionClosedNotification, UserSessionClosedNotificationRef, UserSessionOpenedNotificationRef};

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

pub struct ClientConnection {
  stream: ClientStream,
}

impl ClientConnection {
  pub fn connect(
    path: PathBuf,
    password: &str, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, EstablishConnectionError> {
    let stream = Stream::connect(
      path, 
      // Some random-butt number for
      765765765675, 
      textual_error,
    );
    
    let stream = ClientStream::construct();
  }
}
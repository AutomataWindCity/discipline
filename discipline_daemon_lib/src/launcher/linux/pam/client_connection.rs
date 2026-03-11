use std::path::PathBuf;
use crate::IsTextualError;
use super::super::UserNameRef;
use super::{Stream, EstablishConnectionRef, EstablishConnectionReply, EstablishConnectionError, IsUserSessionOpenBlockedRef, IsUserSessionOpenBlockedReply, UserSessionClosedNotificationRef, UserSessionOpenedNotificationRef, AuthenticationToken};

pub struct ClientStream {
  stream: Stream,
}

impl ClientStream {
  pub fn construct(stream: Stream) -> Self {
    Self { stream }
  }

  pub fn send_establish_connection(
    &mut self, 
    authentication_token: &AuthenticationToken, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(
      &EstablishConnectionRef {
        authentication_token,
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

  pub fn send_is_user_session_open_blocked(
    &mut self,
    user_name: UserNameRef<'_>,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.send(
      &IsUserSessionOpenBlockedRef {
        user_name,
      },
      textual_error,
    )    
  }

  pub fn recv_is_user_session_open_blocked_reply(
    &mut self,
    textual_error: &mut impl IsTextualError
  ) -> Result<IsUserSessionOpenBlockedReply, ()> {
    self.stream.recv(textual_error)
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
  is_closed: bool,
}

impl ClientConnection {
  pub fn connect(
    path: &PathBuf,
    authentication_token: &AuthenticationToken, 
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
    
    if let Err(()) = stream.send_establish_connection(authentication_token, textual_error) {
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
      EstablishConnectionReply::UnrecognizedAuthenticationToken => {
        Err(EstablishConnectionError::UnrecognizedAuthenticationToken)
      }
      EstablishConnectionReply::ConnectionEstablished => {
        Ok(ClientConnection { 
          stream,
          is_closed: false,
        })
      }
    }
  }

  pub fn is_user_session_open_blocked(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<bool, ()> {
    let mut textual_error = textual_error.optional_context("Discipline Linux-PAM Module Client sending an IsUserSessionOpenPermitted message");

    if self.is_closed {
      // TODO: Add more info.
      textual_error.add_message("The client is closed due to an eariler fatal io error");
      return Err(());
    }

    if let Err(()) = self.stream.send_is_user_session_open_blocked(user_name, &mut textual_error) {
      self.is_closed = true;
      return Err(());
    }

    let reply = match self.stream.recv_is_user_session_open_blocked_reply(&mut textual_error) {
      Ok(value) => {
        value
      }
      Err(()) => {
        self.is_closed = true;
        return Err(());
      }
    };

    Ok(reply.is_user_session_open_blocked)
  }

  pub fn send_user_session_opened_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error.optional_context("Discipline Linux-PAM Module Client sending a UserSessionOpenedNotification");

    if self.is_closed {
      // TODO: Add more info.
      textual_error.add_message("The client is closed due to an eariler fatal io error");
      return Err(());
    }

    if let Err(()) = self.stream.send_user_session_opened_notification(user_name, &mut textual_error) {
      self.is_closed = true;
      return Err(());
    }

    return Ok(());
  }

  pub fn send_user_session_closed_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error.optional_context("Discipline Linux-PAM Module Client sending a UserSessionClosedNotification");

    if self.is_closed {
      // TODO: Add more info.
      textual_error.add_message("The client is closed due to an eariler fatal io error");
      return Err(());
    }

    if let Err(()) = self.stream.send_user_session_opened_notification(user_name, &mut textual_error) {
      self.is_closed = true;
      return Err(());
    }

    return Ok(());
  }
}
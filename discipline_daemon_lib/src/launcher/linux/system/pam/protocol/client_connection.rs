use std::path::PathBuf;
use crate::x::IsTextualError;
use super::{
  UserNameRef, 
  BlockingStream, 
  BufferLength, 
  BincodeSerializationFormat, 
  EstablishConnectionRef, 
  EstablishConnectionReply, 
  EstablishConnectionError, 
  IsUserSessionOpenBlockedRef, 
  ClientMessageRef,
  IsUserSessionOpenBlockedReply, 
  UserSessionClosedNotificationRef, 
  UserSessionOpenedNotificationRef, 
  AuthenticationToken,
};


// TODO
pub const MAXIMUM_MESSAGE_LENGTH: BufferLength = BufferLength::create_or_panic(7987);

struct ClientStream {
  path: PathBuf,
  stream: BlockingStream,
  closed: bool,
}

impl ClientStream {
  pub fn construct(
    path: PathBuf,
    stream: BlockingStream,
  ) -> Self {
    Self {
      path,
      stream,
      closed,
    }
  }

  fn ensure_connected(
    &mut self,
    path: &Path,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    if self.closed {
      self.stream.reconnect(
        &self.path, 
        &mut textual_error.change_context("Discipline Linux-PAM Client Stream reconnecting after closing due to an eariler error"),
      )?;
    }
    
    Ok(())
  }

  fn shutdown(
    &mut self, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self.stream.shutdown(textual_error)
  }

  fn write_establish_connection(
    &mut self, 
    authentication_token: &AuthenticationToken, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    self 
      .stream 
      .ensure_connected(
        &self.path, 
        &mut textual_error,
      )?;

    self.stream.write(
      &EstablishConnectionRef { authentication_token }, 
      &BincodeSerializationFormat,
      textual_error,
    )
  }

  fn read_establish_connection_repl(
    &mut self,
    textual_error: &mut impl IsTextualError
  ) -> Result<EstablishConnectionReply, ()> {
    if let Err(()) = self.ensure_connected(&self.path, &mut textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }
    
    let format = BincodeSerializationFormat;

    match self.stream.read(&format, textual_error) {
      Ok(value) => {
        Ok(value)
      }
      Err(()) => {
        self.shutdown(textual_error)?;
        return Err(());
      }
    }
  }

  fn write_is_user_session_open_blocked(
    &mut self,
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    if let Err(()) = self .stream .ensure_connected(&self.path, &mut textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    let format = BincodeSerializationFormat;
    let message = ClientMessageRef::IsUserSessionOpenBlocked(
      IsUserSessionOpenBlockedRef { 
        user_name,
      }
    );

    if let Err(()) = self.stream.write(&message, &format, textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    Ok(())
  }

  fn read_is_user_session_open_blocked_reply(
    &mut self,
    textual_error: &mut impl IsTextualError
  ) -> Result<IsUserSessionOpenBlockedReply, ()> {
    if let Err(()) = self .stream .ensure_connected(&self.path, &mut textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    let format = BincodeSerializationFormat;

    match self.stream.read(&format, textual_error) {
      Ok(value) => {
        Ok(value)
      }
      Err(()) => {
        self.shutdown(textual_error)?;
        Err(())
      }
    }
  }

  fn write_user_session_opened_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    if let Err(()) = self .stream .ensure_connected(&self.path, &mut textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    let format = BincodeSerializationFormat;

    let message = ClientMessageRef::UserSessionOpenedNotification(
      UserSessionOpenedNotificationRef {
        user_name,
      }
    );
    
    if let Err(()) = self.stream.write(&message, &format, textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    Ok(())
  }

  fn write_user_session_closed_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    if let Err(()) = self .stream .ensure_connected(&self.path, &mut textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    let format = BincodeSerializationFormat;
    let message = ClientMessageRef::UserSessionClosedNotification(
      UserSessionClosedNotificationRef {
        user_name,
      }
    );

    if let Err(()) = self.stream.write(&message, &format, textual_error) {
      self.shutdown(textual_error)?;
      return Err(());
    }

    Ok(())
  }
}

// struct ClientStream {
//   stream: BlockingStream,
// }

// impl ClientStream {
//   pub fn construct(stream: BlockingStream) -> Self {
//     Self { stream }
//   }

//   fn write_establish_connection(
//     &mut self, 
//     authentication_token: &AuthenticationToken, 
//     textual_error: &mut impl IsTextualError,
//   ) -> Result<(), ()> {
//     self.stream.write(
//       &EstablishConnectionRef { authentication_token }, 
//       &BincodeSerializationFormat,
//       textual_error,
//     )
//   }

//   fn read_establish_connection_repl(
//     &mut self,
//     textual_error: &mut impl IsTextualError
//   ) -> Result<EstablishConnectionReply, ()> {
//     self.stream.read(
//       &BincodeSerializationFormat,
//       textual_error,
//     )
//   }

//   fn write_is_user_session_open_blocked(
//     &mut self,
//     user_name: UserNameRef,
//     textual_error: &mut impl IsTextualError,
//   ) -> Result<(), ()> {
//     self.stream.write(
//       &ClientMessageRef::IsUserSessionOpenBlocked(
//         IsUserSessionOpenBlockedRef { 
//           user_name,
//         }
//       ),
//       &BincodeSerializationFormat,
//       textual_error,
//     )    
//   }

//   fn read_is_user_session_open_blocked_reply(
//     &mut self,
//     textual_error: &mut impl IsTextualError
//   ) -> Result<IsUserSessionOpenBlockedReply, ()> {
//     self.stream.read(
//       &BincodeSerializationFormat,
//       textual_error,
//     )
//   }

//   fn write_user_session_opened_notification(
//     &mut self, 
//     user_name: UserNameRef,
//     textual_error: &mut impl IsTextualError,
//   ) -> Result<(), ()> {
//     self.stream.write(
//       &ClientMessageRef::UserSessionOpenedNotification(
//         UserSessionOpenedNotificationRef {
//           user_name,
//         }
//       ),
//       &BincodeSerializationFormat,
//       textual_error,
//     )
//   }

//   fn write_user_session_closed_notification(
//     &mut self, 
//     user_name: UserNameRef,
//     textual_error: &mut impl IsTextualError,
//   ) -> Result<(), ()> {
//     self.stream.write(
//       &ClientMessageRef::UserSessionClosedNotification(
//         UserSessionClosedNotificationRef {
//           user_name,
//         }
//       ),
//       &BincodeSerializationFormat,
//       textual_error,
//     )
//   }

//   fn ensure_connected(
//     &mut self,
//     path: &Path,
//     textual_error: &mut impl IsTextualError,
//   ) -> Result<(), ()> {
//     self.stream.reconnect(path, textual_error)
//   }
// }

pub struct ClientConnection {
  path: PathBuf,
  stream: ClientStream,
  closed: bool,
}

impl ClientConnection {
  pub fn connect(
    path: PathBuf,
    authentication_token: &AuthenticationToken, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, EstablishConnectionError> {
    let stream = match BlockingStream::connect(
      path, 
      maximum_buffer_length, 
      textual_error,
    ) {
      Ok(value) => {
        value
      }
      Err(()) => {
        return Err(EstablishConnectionError::Other);
      }
    };

    let mut stream = ClientStream::construct(
      path, 
      stream,
    );
    
    if let Err(()) = stream.write_establish_connection(authentication_token, textual_error) {
      return Err(EstablishConnectionError::Other);
    }

    let reply = match stream.read_establish_connection_repl(textual_error) {
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
          path,
          closed: false,
        })
      }
    }
  }

  pub fn is_user_session_open_blocked(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<bool, ()> {
    let mut textual_error = textual_error
      .optional_context("Discipline Linux-PAM Module Client sending an IsUserSessionOpenPermitted message");

    self
      .stream
      .write_is_user_session_open_blocked(user_name, &mut textual_error)?;

    let reply = self
      .stream
      .read_is_user_session_open_blocked_reply(&mut textual_error)?;

    Ok(reply.is_user_session_open_blocked)
  }

  pub fn send_user_session_opened_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error
      .optional_context("Discipline Linux-PAM Module Client sending a UserSessionOpenedNotification");

    self
      .stream
      .write_user_session_opened_notification(user_name, &mut textual_error) 
  }

  pub fn send_user_session_closed_notification(
    &mut self, 
    user_name: UserNameRef,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error
      .optional_context("Discipline Linux-PAM Module Client sending a UserSessionClosedNotification");

    self
      .stream
      .write_user_session_opened_notification(user_name, &mut textual_error) 
  }
}
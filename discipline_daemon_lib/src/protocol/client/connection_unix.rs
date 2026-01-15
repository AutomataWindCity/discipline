use std::io::Write;
use std::{io::Read, os::unix::net::UnixStream};
use std::path::{Path, PathBuf};
use crate::TextualError;
use crate::operating_system::UserNameRef;
use std::any::type_name;
use serde::Deserialize;
use serde::{Serialize, de::DeserializeOwned};

type BincodeConfiguration = bincode
  ::config
  ::Configuration<
    bincode::config::BigEndian, 
    bincode::config::Fixint, 
    bincode::config::Limit<500>
  >
;

static BINCODE_CONFIG: BincodeConfiguration = bincode::config::standard()
  .with_big_endian()
  .with_fixed_int_encoding()
  .with_limit();

pub trait Serializable {
  type Error;

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait Deserializable: Sized {
  type Error;

  fn deserialize(buffer: &[u8]) -> Result<Self, Self::Error>;
}


impl<T> Serializable for T 
where 
  T: Serialize
{
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    bincode::serde::encode_into_slice(
      value, 
      buffer, 
      BINCODE_CONFIG,
    ).map_err(|_error| {
      ()
    })
  }
}


impl<'de, T> Deserializable for T 
where 
  T: DeserializeOwned
{
  type Error = ();

  fn deserialize(buffer: &[u8]) -> Result<Self, Self::Error> {
    let (value, bytes_read) = bincode::serde::decode_from_slice(
      buffer, 
      BINCODE_CONFIG,
    ).map_err(|_error| {
      ()
    })?;

    if bytes_read != buffer.len() {
      return Err(());
    }
    
    Ok(value)
  }
}


pub fn serialize<T>(value: &T, buffer: &mut [u8]) -> Result<usize, TextualError> 
where 
  T: Serialize
{
  bincode::serde::encode_into_slice(
    value, 
    buffer, 
    BINCODE_CONFIG,
  ).map_err(|error| {
    TextualError::new(format!("Serializing {} using bincode", type_name::<T>()))
      .with_attachement_display("Error", error)
  })
}

pub fn deserialize<T>(buffer: &[u8]) -> Result<T, TextualError> 
where 
  T: DeserializeOwned
{
  let (value, bytes_read) = bincode::serde::decode_from_slice(
    buffer, 
    BINCODE_CONFIG,
  ).map_err(|error| {
    // TODO: Make error message mre generic
    TextualError::new(format!("Deserializing a datagram message containing a value of type {} using bincode", type_name::<T>()))
      .with_message("An error occured duraing deserialization")
      .with_attachement_display("Error", error)
  })?;

  if bytes_read != buffer.len() {
    return Err(
      TextualError::new(format!("Deserializing a datagram message containing a value of type {} using bincode", type_name::<T>()))
        .with_message("The deserialized value length isn't the same as the length reported by the datagram metadata")
        .with_attachement_display("Datagram message length", buffer.len())
        .with_attachement_display("Deserialized value length", bytes_read)
    );
  }
  
  Ok(value)
}

trait IsLogger {
  fn info(&mut self, str: &str);
  fn error(&mut self, str: &str);
  fn debug(&mut self, str: &str);
}


struct Timeout {
  milliseconds: i32
}

const MAXIMUM_MESSAGE_LENGTH: usize = 89;
const MESSAGE_LENGTH_SIZE: usize = 4;

struct Connection {
  buffer: Box<[u8]>,
  connection: UnixStream,
  is_closed: bool,
  path: PathBuf,
}

impl Connection {
  // pub fn connect(path: impl AsRef<Path>, timeout: &mut Timeout) -> Result<Self, ()> {    
  //   let socket = unsafe {
  //     libc::socket(
  //       libc::AF_UNIX, 
  //       libc::SOCK_STREAM, 
  //       0,
  //     )
  //   };

  //   if socket < 0 {
  //     return Err(());
  //   }

  //   // Set non-blocking
  //   let flags = unsafe {
  //     libc::fcntl(socket, libc::F_GETFL)
  //   };

  //   if flags < 0 {
  //     return Err(());
  //   }

  //   let result = unsafe {
  //     libc::fcntl(
  //       socket, 
  //       libc::F_SETFL, 
  //       flags | libc::O_NONBLOCK,
  //     )
  //   };

  //   if result < 0 {
  //     return Err(());
  //   }
    
  //   todo!()
  // }
}

enum Error {
  
}

impl Connection {
  pub fn connect(path: PathBuf) -> Result<Self, ()> {
    let _ = std::fs::remove_file(&path);

    match UnixStream::connect(&path) {
      Ok(connection) => {
        Ok(Self {
          buffer: Box::new([0; MESSAGE_LENGTH_SIZE + MAXIMUM_MESSAGE_LENGTH]),
          connection,
          is_closed: false,
          path,
        })
      }
      Err(error) => {
        Err(())
      }
    }
  }

  fn ensure_connected(&mut self) -> Result<(), ()> {
    if !self.is_closed {
      return Ok(());
    }

    let _ = std::fs::remove_file(&path);

    match UnixStream::connect(&self.path) {
      Ok(connection) => {
        self.connection = connection;
        Ok(())
      }
      Err(error) => {
        Err(())
      }
    }
  }

  fn shutdown(&mut self) {
    self.connection.shutdown(std::net::Shutdown::Both);
    self.is_closed = true;
  }

  fn read<T>(&mut self) -> Result<T, ()> 
  where 
    T: DeserializeOwned
  {
    let mut message_length: [u8; 4] = [0; 4];

    if let Err(error) = self
      .connection
      .read(&mut message_length)
    {
      self.shutdown();
      return Err(());
    }

    let message_length = u32::from_be_bytes(message_length) as usize;

    let mut message = Vec::with_capacity(message_length);

    if let Err(error) = self
      .connection
      .read(&mut message)
    {
      self.shutdown();
      return Err(());
    }

    match deserialize(&message) {
      Ok(value) => {
        Ok(value)
      }
      Err(error) => {
        self.shutdown();
        Err(())
      }
    }
  }
  
  fn write<T>(&mut self, value: &T) -> Result<(), ()> 
  where 
    T: Serialize
  {
    let message_buffer = &mut self.buffer[MESSAGE_LENGTH_SIZE..];

    let message_length = match serialize(value, message_buffer) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(());
      }
    };

    if message_length > MAXIMUM_MESSAGE_LENGTH {
      return Err(());
    }

    let message_length_as_u32: u32 = match message_length.try_into() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(());
      }
    };

    let message_length_as_bytes = message_length_as_u32.to_be_bytes();
    let message_length_buffer = &mut self.buffer[..MESSAGE_LENGTH_SIZE];
    message_length_buffer.copy_from_slice(&message_length_as_bytes);

    let packet_length = MESSAGE_LENGTH_SIZE + message_length;
    let packet = &self.buffer[0..packet_length];

    if let Err(error) = self
      .connection
      .write(&packet)
    {
      self.shutdown();
      return Err(());
    }

    Ok(())
  }
}

pub enum SendErrorCode {
  ErrorWhileSerializingDatagramContent,
  DatagramContentLengthLargerThanMaximumLength,
  IoErrorWhileWritingDatagram(std::io::Error),
}

pub enum RecvErrorCode {
  IoErrorWhileReadingDatagramContentLength(std::io::Error),
  DatagramContentLengthLargerThanMaximumLength(usize),
  IoErrorWhileReadingDatagramContent(std::io::Error),
  ErrorWhileDeserializingDatagramContent,
}

pub struct DatagramConnection {
  connection: UnixStream,
  datagram_content_buffer: Vec<u8>,
}

impl DatagramConnection {
  // connect_with_logging
  

  pub async fn connect_or_textual_error(
    path: impl AsRef<Path>, 
    maximum_message_length: usize,
  ) -> Result<DatagramConnection, TextualError> {
    UnixStream::connect(path)
      .await
      .map(|connection| Self::construct(
        connection, 
        maximum_message_length
      ))
      .map_err(|error| {
        TextualError::new("Connecting to a Unix Datagram")
          .with_message("An io error occured")
          .with_attachement_display("Io error", error)
      })
  }

  pub async fn connect_or_error(path: impl AsRef<Path>, maximum_length: usize) -> Result<DatagramConnection, std::io::Error> {
    UnixStream::connect(path)
      .await
      .map(|connection| Self::construct(
        connection, 
        maximum_length
      ))
  }

  pub fn construct(connection: UnixStream, maximum_message_length: usize) -> Self {
    Self {
      connection,
      datagram_content_buffer: vec![0; maximum_message_length],
    }
  }

  fn maximum_content_length(&self) -> usize {
    self.datagram_content_buffer.len()
  }

  pub async fn recv<T>(&mut self) -> Result<T, ()> 
  where 
    T: DeserializeOwned
  {
    let mut content_length = [0; DATAGRAM_CONTENT_LENGTH_SIZE];

    if let Err(error) = self.connection.read_exact(&mut content_length).await {
      return Err(());
    }
    
    let content_length = u32::from_be_bytes(content_length) as usize;
    if content_length > self.maximum_content_length() {
      return Err(());
    }

    let message_content = &mut self.datagram_content_buffer[..content_length];

    if let Err(_) = self.connection.read_exact(message_content).await {
      return Err(());
    }

    deserialize(message_content).map_err(|_| ())
  }

  pub async fn send<T>(&mut self, content: &T) -> Result<(), ()> 
  where 
    T: Serialize
  {
    let content_buffer = &mut self.datagram_content_buffer[DATAGRAM_CONTENT_LENGTH_SIZE..];

    let Ok(content_length) = serialize(content, content_buffer) else {
      return Err(());
    };
    
    let Ok(content_length_as_u32): Result<u32, _> = content_length.try_into() else {
      return Err(());
    };

    let content_length_buffer = &mut self.datagram_content_buffer[0..DATAGRAM_CONTENT_LENGTH_SIZE];
    content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

    let datagram = &self.datagram_content_buffer[
      ..
      DATAGRAM_CONTENT_LENGTH_SIZE + content_length
    ];

    self.connection
      .write_all(&datagram)
      .await
      .map_err(|_| {
        ()
      })?;

    Ok(())
  }

  pub async fn send_or_error<T>(&mut self, content: &T) -> Result<(), SendErrorCode> 
  where 
    T: Serialize
  {
    let content_buffer = &mut self.datagram_content_buffer[DATAGRAM_CONTENT_LENGTH_SIZE..];

    let Ok(content_length) = serialize(content, content_buffer) else {
      return Err(SendErrorCode::ErrorWhileSerializingDatagramContent);
    };
    
    if content_length > self.maximum_content_length() {
      return Err(SendErrorCode::DatagramContentLengthLargerThanMaximumLength);
    }

    let Ok(content_length_as_u32): Result<u32, _> = content_length.try_into() else {
      return Err(SendErrorCode::DatagramContentLengthLargerThanMaximumLength);
    };

    let content_length_buffer = &mut self.datagram_content_buffer[0..DATAGRAM_CONTENT_LENGTH_SIZE];
    content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

    let datagram = &self.datagram_content_buffer[
      ..
      DATAGRAM_CONTENT_LENGTH_SIZE + content_length
    ];

    self.connection
      .write_all(&datagram)
      .await
      .map_err(|error| {
        SendErrorCode::IoErrorWhileWritingDatagram(error)
      })
  }

  pub async fn recv_or_error<T>(&mut self) -> Result<T, RecvErrorCode> 
  where 
    T: DeserializeOwned
  {
    let mut content_length = [0; DATAGRAM_CONTENT_LENGTH_SIZE];

    self
      .connection
      .read_exact(&mut content_length)
      .await
      .map_err(|error| {
        RecvErrorCode::IoErrorWhileReadingDatagramContentLength(error)
      })?;
    
    let content_length = u32::from_be_bytes(content_length) as usize;
    if content_length > self.maximum_content_length() {
      return Err(RecvErrorCode::DatagramContentLengthLargerThanMaximumLength(content_length));
    }

    let message_content = &mut self.datagram_content_buffer[..content_length];

    self
      .connection
      .read_exact(message_content)
      .await
      .map_err(|error| {
        RecvErrorCode::IoErrorWhileReadingDatagramContent(error)
      })?;

    deserialize(message_content)
      .map_err(|_| {
        RecvErrorCode::ErrorWhileDeserializingDatagramContent
      })
  }

  pub async fn send_or_textual_error<T>(&mut self, content: &T) -> Result<(), TextualError> 
  where 
    T: Serialize
  {
    let content_buffer = &mut self.datagram_content_buffer[DATAGRAM_CONTENT_LENGTH_SIZE..];

    let content_length = serialize(content, content_buffer).map_err(|error| {
      error
        .with_context("Sending a message over a UnixStream")
        .with_message("Failed to serialize the message")
        .with_attachement_display("Message data type name", type_name::<T>())
        .with_attachement_display("Message buffer length", self.datagram_content_buffer.len())
    })?;
    
    if content_length > self.maximum_content_length() {
      return Err(
        TextualError::new("Sending a message over a UnixStream")
          .with_message("Message length is larger than the maximum allowed length")
          .with_attachement_display("Message data type name", type_name::<T>())
          .with_attachement_display("Message length", content_length)
          .with_attachement_display("Message maximum valid length", self.maximum_content_length())

      );
    }

    let Ok(content_length_as_u32): Result<u32, _> = content_length.try_into() else {
      return Err(
        TextualError::new("Sending a message over a UnixStream")
          .with_message("Message length is larger than the maximum allowed length")
          .with_attachement_display("Message data type name", type_name::<T>())
          .with_attachement_display("Message length", content_length)
          .with_attachement_display("Message maximum valid length", self.maximum_content_length())

      );
    };

    let content_length_buffer = &mut self.datagram_content_buffer[0..DATAGRAM_CONTENT_LENGTH_SIZE];
    content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

    let datagram = &self.datagram_content_buffer[
      ..
      DATAGRAM_CONTENT_LENGTH_SIZE + content_length
    ];

    self.connection
      .write_all(&datagram)
      .await
      .map_err(|error| {
        TextualError::new("Sending a message over a UnixStream")
          .with_message("An io error occured")
          .with_attachement_display("Message data type name", type_name::<T>())
          .with_attachement_display("Io error", error)

      })
  }

  pub async fn recv_or_textual_error<T>(&mut self) -> Result<T, TextualError> 
  where 
    T: DeserializeOwned
  {
    let mut content_length = [0; DATAGRAM_CONTENT_LENGTH_SIZE];

    self
      .connection
      .read_exact(&mut content_length)
      .await
      .map_err(|error| {
        TextualError::new("Receiving a message over a UnixStream")
          .with_message("An io error occured while reading the message length")
          .with_attachement_display("Message data type", type_name::<T>())
          .with_attachement_display("Io error", error)
      })?;
    
    let content_length = u32::from_be_bytes(content_length) as usize;
    if content_length > self.maximum_content_length() {
      return Err(
        TextualError::new("Receiving a message over a UnixStream")
          .with_message("Message length is larger than maximum valid length")
          .with_attachement_display("Message data type", type_name::<T>())
          .with_attachement_display("Message length", content_length)
          .with_attachement_display("Message maximum valid length", self.maximum_content_length())
      );
    }

    let message_content = &mut self.datagram_content_buffer[..content_length];

    self
      .connection
      .read_exact(message_content)
      .await
      .map_err(|error| {
        TextualError::new("Receiving a message over a UnixStream")
          .with_message("An io error occured while reading the message")
          .with_attachement_display("Message data type", type_name::<T>())
          .with_attachement_display("Io error", error)
      })?;

    deserialize(message_content)
      .map_err(|error| {
        TextualError::new("Receiving a message over a UnixStream")
          .with_message("An error occured while deserializing the message")
          .with_attachement_display("Message data type", type_name::<T>())
          .with_attachement_display("Deserialization error", error)
      })
  }
}
pub struct ClientConnection {
  connection: DatagramConnection
}

impl ClientConnection {
  pub async fn connect_or_textual_error(
    path: impl AsRef<Path>, 
    password: &str,
  ) -> Result<ClientConnection, TextualError> {
    let mut connection = DatagramConnection
      ::connect_or_textual_error(path, MAXIMUM_MESSAGE_LENGTH)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module connecting to Discipline Daemon")
      })?;

    let establish_connection = EstablishConnectionRef {
      password,
    };

    connection
      .send_or_textual_error(&establish_connection)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module connecting to Discipline Daemon")
          .with_message("An io error ocuured while sending the EstablishConnection message")
      })?;

    let establish_connection_reply: EstablishConnectionReply = connection
      .recv_or_textual_error()
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module connecting to Discipline Daemon")
          .with_message("An io error ocuured while receiving the EstablishConnectionReply message")
      })?;

    match establish_connection_reply {
      EstablishConnectionReply::ServerBusy => {
        Err(
          TextualError::new("Discipline Linux-PAM Module connecting to Discipline Daemon")
            .with_message("Discipline Daemon is busy")
        )
      }
      EstablishConnectionReply::IncorrectPassword => {
        Err(
          TextualError::new("Discipline Linux-PAM Module connecting to Discipline Daemon")
            .with_message("Discipline Daemon rejected the connection because it didn't recognize our credintials")
        )
      }
      EstablishConnectionReply::ConnectionEstablished => {
        Ok(Self { 
          connection 
        })
      }
    }
  }

  pub async fn is_user_session_open_permitted_or_textual_error(
    &mut self, 
    user_name: UserNameRef<'_>,
  ) -> Result<bool, TextualError> {
    let message = ClientMessageRef::IsUserSessionOpenPermitted(
      IsUserSessionOpenPermittedRef { 
        user_name,
      }
    );

    self
      .connection
      .send_or_textual_error(&message)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM sending an IsUserSessionOpenPermitted message to Discipline Daemon")
      })?;

    let reply: IsUserSessionOpenPermittedReply = self
      .connection
      .recv_or_textual_error()
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module sending an IsUserSessionOpenPermitted notification to Discipline Daemon")
          .with_message("An error occured while receiving an IsUserSessionOpenPermittedReply")  
      })?;

    Ok(reply.is_user_session_open_permitted)
  }

  pub async fn send_user_session_opened_notification_or_textual_error(
    &mut self, 
    user_name: UserNameRef<'_>,
  ) -> Result<(), TextualError> {
    let message = ClientMessageRef::UserSessionOpenedNotification(
      UserSessionOpenedNotificationRef { 
        user_name,
      }
    );

    self
      .connection
      .send_or_textual_error(&message)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module sending a UserSessionOpenedNotification to Discipline Daemon")
      })
  }

  pub async fn send_user_session_closed_notification_or_textual_error(
    &mut self, 
    user_name: UserNameRef<'_>,
  ) -> Result<(), TextualError> {
    let message = ClientMessageRef::UserSessionClosedNotification(
      UserSessionClosedNotificationRef { 
        user_name, 
      }
    );

    self
      .connection
      .send_or_textual_error(&message)
      .await
      .map_err(|error| {
        error
          .with_context("Discipline Linux-PAM Module sending a UserSessionClosedNotification to Discipline Daemon")
      })
  }

}

  // pub async fn connect(
  //   path: &Path, 
  //   password: &String,
  // ) -> Result<Self, TextualError> {
  //   let connection = UnixStream::connect(path)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Linux-PAM Module connecting to discipline server")
  //         .with_message("An io error occured when establishing UnixStream connection")
  //         .with_attachement_display("Server path", path.display())
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   let message = ClientEstablishConnectionRequest {
  //     password: password.clone(),
  //   };

  //   let message = serialize(&message).map_err(|error| {
  //     error.with_context("Linux-PAM Module connecting to discipline")
  //   })?;

  //   connection
  //     .write_all(&message)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Linux-PAM Module connectiong to discipline server")
  //         .with_message("An io error occured when writing the connection preface")
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   // TODO: Recieve a confirmation message
  //   // TODO: Revieve a password and verify it.

  //   return Ok(Self { 
  //     connection,
  //     buffer: Box::new([0; MAXIMUM_CONTENT_LENGTH]),
  //   });
  // }

  // pub async fn is_user_login_blocked(
  //   &mut self, 
  //   operating_system_user_name: &operating_system::UserName,
  // ) -> Result<bool, ()> {
  //   if let Err(_) = write_message(
  //     &mut self.connection, 
  //     &mut *self.buffer, 
  //     &ClientMessage::IsUserLoginBlocked(operating_system_user_name.clone()),
  //   ).await {
  //     return Err(());
  //   }

  //   todo!()
  // }

  // pub fn notify_that_user_session_opened(
  //   &mut self, 
  //   operating_system_user_name: &operating_system::UserName,
  // ) -> Result<(), TextualError> {
  //   todo!()
  // }

  // pub fn notify_that_user_session_closed(
  //   &mut self, 
  //   operating_system_user_name: &operating_system::UserName,
  // ) -> Result<(), TextualError> {
  //   todo!()
  // }

  // pub async fn connect(
  //   path: impl AsRef<Path>, 
  //   password: &str,
  // ) -> Result<ClientConnection, ()> {
  //   let Ok(mut connection) = DatagramConnection::connect(path, MAXIMUM_MESSAGE_LENGTH).await else {
  //     return Err(());
  //   };

  //   let establish_connection = EstablishConnectionRef {
  //     password,
  //   };

  //   let Ok(_) = connection.send(&establish_connection).await else {
  //     return Err(());
  //   };

  //   let Ok(establish_connection_reply) = connection.recv::<EstablishConnectionReply>().await else {
  //     return Err(());
  //   };

  //   match establish_connection_reply {
  //     EstablishConnectionReply::ServerBusy => {
  //       Err(())
  //     }
  //     EstablishConnectionReply::IncorrectPassword => {
  //       Err(())
  //     }
  //     EstablishConnectionReply::ConnectionEstablished => {
  //       Ok(Self { connection })
  //     }
  //   }
  // }

  // pub async fn is_user_session_open_permitted(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<bool, ()> {
  //   let message = ClientMessageRef::IsUserSessionOpenPermitted(
  //     IsUserSessionOpenPermittedRef { 
  //       user_name,
  //     }
  //   );

  //   let Ok(_) = self.connection.send(&message).await else {
  //     return Err(())
  //   };

  //   let Ok(reply) = self.connection.recv::<IsUserSessionOpenPermittedReply>().await else {
  //     return Err(());
  //   };

  //   Ok(reply.is_user_session_open_permitted)
  // }

  // pub async fn send_user_session_opened_notification(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<(), ()> {
  //   let message = ClientMessageRef::UserSessionOpenedNotification(
  //     UserSessionOpenedNotificationRef { 
  //       user_name,
  //     }
  //   );

  //   self
  //     .connection
  //     .send(&message)
  //     .await
  //     .map_err(|_| {
  //       ()
  //     })
  // }

  // pub async fn send_user_session_closed_notification(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<(), ()> {
  //   let message = ClientMessageRef::UserSessionClosedNotification(
  //     UserSessionClosedNotificationRef { 
  //       user_name, 
  //     }
  //   );

  //   self
  //     .connection
  //     .send(&message)
  //     .await
  //     .map_err(|_| {
  //       ()
  //     })
  // }

  // pub async fn connect_or_error(
  //   path: impl AsRef<Path>, 
  //   password: &str,
  // ) -> Result<ClientConnection, EstablishConnectionError> {
  //   let mut connection = DatagramConnection::connect_or_error(path, MAXIMUM_MESSAGE_LENGTH)
  //     .await
  //     .map_err(|error| {
  //       EstablishConnectionError::DatagramConnect(error)
  //     })?;

  //   let establish_connection = EstablishConnectionRef {
  //     password,
  //   };

  //   connection
  //     .send_or_error(&establish_connection)
  //     .await
  //     .map_err(|error| {
  //       EstablishConnectionError::SendEstablishConnectionMessage(error)
  //     })?;

  //   let establish_connection_reply: EstablishConnectionReply = connection
  //     .recv_or_error()
  //     .await
  //     .map_err(|error| {
  //       EstablishConnectionError::RecvEstablishConnectionMessage(error)
  //     })?;

  //   match establish_connection_reply {
  //     EstablishConnectionReply::ServerBusy => {
  //       Err(EstablishConnectionError::ServerBusy)
  //     }
  //     EstablishConnectionReply::IncorrectPassword => {
  //       Err(EstablishConnectionError::IncorrectPassword)
  //     }
  //     EstablishConnectionReply::ConnectionEstablished => {
  //       Ok(Self { connection })
  //     }
  //   }
  // }

  // pub async fn is_user_session_open_permitted_or_error(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<bool, IsUserSessionOpenPermittedError> {
  //   let message = ClientMessageRef::IsUserSessionOpenPermitted(
  //     IsUserSessionOpenPermittedRef { 
  //       user_name,
  //     }
  //   );

  //   self
  //     .connection
  //     .send_or_error(&message)
  //     .await
  //     .map_err(|error| {
  //       IsUserSessionOpenPermittedError::SendMessage(error)
  //     })?;

  //   let reply: IsUserSessionOpenPermittedReply = self
  //     .connection
  //     .recv_or_error()
  //     .await
  //     .map_err(|error| {
  //       IsUserSessionOpenPermittedError::RecvReply(error)
  //     })?;

  //   Ok(reply.is_user_session_open_permitted)
  // }

  // pub async fn send_user_session_opened_notification_or_error(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<(), SendErrorCode> {
  //   let message = ClientMessageRef::UserSessionOpenedNotification(
  //     UserSessionOpenedNotificationRef { 
  //       user_name,
  //     }
  //   );

  //   self
  //     .connection
  //     .send_or_error(&message)
  //     .await
  // }

  // pub async fn send_user_session_closed_notification_or_error(
  //   &mut self, 
  //   user_name: UserNameRef<'_>,
  // ) -> Result<(), SendErrorCode> {
  //   let message = ClientMessageRef::UserSessionClosedNotification(
  //     UserSessionClosedNotificationRef { 
  //       user_name, 
  //     }
  //   );

  //   self
  //     .connection
  //     .send_or_error(&message)
  //     .await
  // }



// type BincodeConfiguration = bincode
//   ::config
//   ::Configuration<
//     bincode::config::BigEndian, 
//     bincode::config::Fixint, 
//     bincode::config::Limit<500>
//   >
// ;

// static BINCODE_CONFIG: BincodeConfiguration = bincode::config::standard()
//   .with_big_endian()
//   .with_fixed_int_encoding()
//   .with_limit();

// pub fn serialize<T>(value: &T, output: &mut [u8]) -> Result<usize, TextualError> 
// where 
//   T: Serialize
// {
//   bincode::serde::encode_into_slice(value, output, BINCODE_CONFIG)
//     .map_err(|error| {
//       TextualError::new(format!("Serializing {} using bincode", type_name::<T>()))
//         .with_attachement_display("Error", error)
//     })
// }

// pub fn deserialize<T>(slice: &[u8]) -> Result<T, TextualError>
// where 
//   T: DeserializeOwned
// {
//   let (value, read_bytes) = match bincode::serde::decode_from_slice(slice, BINCODE_CONFIG) {
//     Ok(value) => {
//       value
//     }
//     Err(error) => {
//       return Err(
//         TextualError::new(format!("Deserializing byte array as {} using bincode", type_name::<T>()))
//           .with_attachement_display("Error", error)
//       );
//     }
//   };

//   if read_bytes != slice.len() {
//     return Err(
//       TextualError::new(format!("Deserializing byte array as {} using bincode", type_name::<T>()))
//         .with_message(format!("Bincode deserialized the byte array successfully, but the number of bytes bincode read is not the same as the byte array size, which shouldn't be possible since the byte array is expected to be the binary repreentation of {}, without additional or missing bytes.", type_name::<T>()))
//         .with_attachement_display("Byte array size", slice.len())
//         .with_attachement_debug("Byte array", slice)
//     )
//   }
  
//   Ok(value)
// }


// async fn write_message(
//   connection: &mut UnixStream, 
//   buffer: &mut [u8],
//   message: &ClientMessage,
// ) -> Result<(), ()> {
//   let length_buffer = &mut buffer[0..CONTENT_LENGTH_SIZE];
//   let message_buffer = &mut buffer[CONTENT_LENGTH_SIZE..];

//   let Ok(message_length) = serialize(message, message_buffer) else {
//     return Err(());
//   };
  
//   let Ok(message_length): Result<u32, _> = message_length.try_into() else {
//     return Err(());
//   };

//   length_buffer.copy_from_slice(&message_length.to_be_bytes());

//   let datagram = &buffer[]
//   connection
//     .write_all(&content)
//     .await
//     .map_err(|error| {
//       TextualError::new("Linux-PAM Module client connection writing server message")
//         .with_message("An io error occured while writing the message content")
//         .with_attachement_display("Io error", error)
//     })?;

//   Ok(())

//   // let content_size_buffer = &mut buffer[0..MESSAGE_SIZE_SIZE];
//   // let content_buffer = &mut buffer[MESSAGE_SIZE_SIZE..];

//   // let content_size = serialize(message, content_buffer)
//   //   .map_err(|error| {
//   //     error
//   //       .with_context("Linux-PAM Module client connection write message to Discipline Daemon")
//   //       .with_message("Failed to serialize the message")
//   //   })?
//   // ;
  
//   // let content_size = MESSAGE_SIZE_SIZE + content_size;
//   // let content_size: MessageLength = content_size
//   //   .try_into()
//   //   .map_err(|_| {
//   //     TextualError::new("Linux-PAM Module client connection write message to Discipline Daemon")
//   //       .with_message("Message is too large")
//   //       .with_attachement_display("Message size", content_size)
//   //       .with_attachement_display("Maximum message size", MAXIMUM_MESSAGE_SIZE)
//   //   })?
//   // ;
//   // if content_size > MAXIMUM_MESSAGE_SIZE as u32 {
//   //   return Err(
//   //     TextualError::new("Linux-PAM Module client connection write message to Discipline Daemon")
//   //       .with_message("Message is too large")
//   //       .with_attachement_display("Message size", content_size)
//   //       .with_attachement_display("Maximum message size", MAXIMUM_MESSAGE_SIZE)
//   //   );
//   // }

//   // content_size_buffer
//   //   .copy_from_slice(&content_size.to_be_bytes());

//   // connection
//   //   .write_all(&content)
//   //   .await
//   //   .map_err(|error| {
//   //     TextualError::new("Linux-PAM Module client connection writing server message")
//   //       .with_message("An io error occured while writing the message content")
//   //       .with_attachement_display("Io error", error)
//   //   })?;

//   // Ok(())
// }


// async fn read_message(
//   connection: &mut UnixStream,
//   buffer: &mut [u8],
// ) -> Result<ServerMessage, ()> {
//   let mut content_length = [0; CONTENT_LENGTH_SIZE];

//   if let Err(_) = connection.read_exact(&mut content_length).await {
//     return Err(());
//   }
  
//   let content_length = u32::from_be_bytes(content_length) as usize;
//   if content_length > CONTENT_LENGTH_SIZE {
//     return Err(());
//   }

//   let message_content = &mut buffer[..content_length];

//   if let Err(_) = connection.read_exact(message_content).await {
//     return Err(());
//   }

//   deserialize(&message_content).map_err(|_| {
//     ()
//   })

//   // let mut content_length = 
//   // connection
//   //   .recv(&mut *self.buffer)
//   //   .await
//   //   .map_err(|error| {
//   //     TextualError::new("Linux-PAM Module client connection reading server message")
//   //       .with_message("An io error occured while reading the message content")
//   //       .with_attachement_display("Io error", error)
//   //   })?;

//   // deserialize(&*self.buffer)
//   //   .map_err(|error| {
//   //     error
//   //       .with_context("Linux-PAM Module client connection reading server message")
//   //       .with_message("Failed to deserialize the message content")
//   //   })
// }

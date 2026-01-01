use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixDatagram;
use tokio::net::UnixStream;
use std::path::Path;
use crate::operating_system;
use crate::x::Daemon;
use crate::x::TextualError;
use crate::x::protocol::x::{DatagramConnection, Serializable, Deserializable};
use serde::Serialize;

const MAXIMUM_USERNAME_SIZE: usize = 32; 

// TODO: Include message length
const MAXIMUM_MESSAGE_LENGTH: usize = {
  // Postcard is more compact than bincode
  MAXIMUM_USERNAME_SIZE
  // u8 for message enum variant
  + 1
  // 4 bytes for size prefix
  + 4
};

#[derive(Debug, Serialize, Deserialize)]
struct ClientEstablishConnectionMessage {
  password: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ServerEstablishConnectionReply {
  Busy,
  WrongPassword,
  ConnectionEstablished,
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessage {
  UserSessionOpened(operating_system::UserName),
  UserSessionClosed(operating_system::UserName),
  IsUserLoginBlocked(operating_system::UserName),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerIsUserLoginBlockedReply {
  is_user_login_blocked: bool,
}

pub struct ClientConnection {
  connection: DatagramConnection
}

impl ClientConnection {
  pub async fn connect(path: impl AsRef<Path>, password: &String) -> Result<ClientConnection, ()> {
    let Ok(mut connection) = DatagramConnection::connect(path, MAXIMUM_MESSAGE_LENGTH).await else {
      return Err(());
    };

    let establish_connection = ClientEstablishConnectionMessage {
      password: password.clone(),
    };

    let Ok(_) = connection.send(&establish_connection).await else {
      return Err(());
    };

    let Ok(establish_connection_response) = connection.recv::<ServerEstablishConnectionReply>().await else {
      return Err(());
    };

    match establish_connection_response {
      ServerEstablishConnectionReply::Busy => {
        Err(())
      }
      ServerEstablishConnectionReply::WrongPassword => {
        Err(())
      }
      ServerEstablishConnectionReply::ConnectionEstablished => {
        Ok(Self { connection })
      }
    }

  }

  pub async fn is_user_login_blocked(
    &mut self, 
    user_name: &operating_system::UserName,
  ) -> Result<bool, ()> {
    let message = ClientMessage::IsUserLoginBlocked(
      user_name.clone()  
    );

    let Ok(_) = self.connection.send(&message).await else {
      return Err(())
    };

    let Ok(reply) = self.connection.recv::<ServerIsUserLoginBlockedReply>().await else {
      return Err(());
    };

    Ok(reply.is_user_login_blocked)
  }

  pub async fn on_user_session_opened(
    &mut self, 
    user_name: &operating_system::UserName,
  ) -> Result<(), ()> {
    let message = ClientMessage::UserSessionOpened(
      user_name.clone(),
    );

    self
      .connection
      .send(&message)
      .await
      .map_err(|_| {
        ()
      })
  }

  pub async fn on_user_session_closed(
    &mut self, 
    user_name: &operating_system::UserName,
  ) -> Result<(), ()> {
    let message = ClientMessage::UserSessionClosed(
      user_name.clone(),
    );

    self
      .connection
      .send(&message)
      .await
      .map_err(|_| {
        ()
      })
  }
}

const CLIENT_PASSWORD: String = String::new();

pub struct ServerConnection {
  connection: DatagramConnection,
}

impl ServerConnection {
  pub async fn establish(mut connection: DatagramConnection) -> Result<Self, ()> {
    let Ok(message) = connection.recv::<ClientEstablishConnectionMessage>().await else {
      return Err(());
    };

    if message.password != CLIENT_PASSWORD {
      _ = connection.send(&ServerEstablishConnectionReply::WrongPassword).await;
      return Err(());
    }

    if let Err(_) = connection.send(&ServerEstablishConnectionReply::ConnectionEstablished).await {
      return Err(());
    }

    Ok(Self { connection })
  }

  pub async fn recv(&mut self) -> Result<>
}

impl ClientConnection {
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
}


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

impl Serializable for ClientMessage {
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    todo!()
  }
}

impl Deserializable for ClientMessage {
  type Error = ();

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl Serializable for ServerMessage {
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    todo!()
  }
}

impl Deserializable for ServerMessage {
  type Error = ();

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl Serializable for ClientEstablishConnectionMessage {
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    todo!()
  }
}

impl Deserializable for ClientEstablishConnectionMessage {
  type Error = ();

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl Serializable for ServerEstablishConnectionReply {
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    todo!()
  }
}

impl Deserializable for ServerEstablishConnectionReply {
  type Error = ();

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error> {
    todo!()
  }
}
impl Serializable for ServerIsUserLoginBlockedReply {
  type Error = ();

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
    todo!()
  }
}

impl Deserializable for ServerIsUserLoginBlockedReply {
  type Error = ();

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error> {
    todo!()
  }
}
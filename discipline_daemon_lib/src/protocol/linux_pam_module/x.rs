use std::{any::type_name, path::Path};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use crate::TextualError;

const DATAGRAM_CONTENT_LENGTH_SIZE: usize = size_of::<u32>();

pub struct DatagramConnection {
  connection: UnixStream,
  datagram_content_buffer: Vec<u8>,
}

impl DatagramConnection {
  // connect_with_logging
  pub async fn connect(path: impl AsRef<Path>, maximum_length: usize) -> Result<DatagramConnection, ()> {
    UnixStream::connect(path)
      .await
      .map(|connection| Self::construct(
        connection, 
        maximum_length
      ))
      .map_err(|_| {
        ()
      })
  }

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
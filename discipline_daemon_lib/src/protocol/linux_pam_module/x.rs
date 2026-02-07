use std::{any::type_name, io::Read, path::Path};
use serde::{Serialize, de::DeserializeOwned};
// use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use std::os::unix::net::UnixStream;
use crate::{IsTextualError, TextualError, protocol::serialization::{deserialize, serialize}};

const MESSAGE_LENGTH_SIZE: usize = size_of::<u32>();

pub struct Stream {
  stream: UnixStream,
  message_buffer: Vec<u8>,
}

enum StreamConnectError {

}

impl Stream {
  pub fn construct(connection: UnixStream, maximum_message_length: usize) -> Self {
    Self {
      stream: connection,
      message_buffer: vec![0; maximum_message_length],
    }
  }
  
  pub fn connect(
    path: impl AsRef<Path>, 
    maximum_message_length: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Stream, ()> {
    match UnixStream::connect(path) {
      Ok(stream) => {
        Ok(Stream {
          stream,
          message_buffer: vec![0; maximum_message_length],
        })
      }
      Err(error) => {
        textual_error.change_context("Connecting to a Unix Stream");
        textual_error.add_message("An io error occured");
        textual_error.add_attachement_display("Io error", error);
        textual_error.add_attachement_display("Unix Stream path", path.as_ref().display());
        return Err(());
      }
    }
  }

  fn maximum_content_length(&self) -> usize {
    self.message_buffer.len()
  }

  pub async fn recv<T>(
    &mut self, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<T, ()> 
  where 
    T: DeserializeOwned
  {
    let mut textual_error = textual_error.optional_context("Receiving a message over a UnixStream");

    let mut message_length = [0; MESSAGE_LENGTH_SIZE];

    if let Err(error) = self.stream.read_exact(&mut message_length) {
      textual_error.add_message("An io error occured while reading the message length");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      textual_error.add_attachement_display("Io error", error);
      return Err(());
    }
    
    let message_length = u32::from_be_bytes(message_length) as usize;
    if message_length > self.maximum_content_length() {
      textual_error.add_message("Message length is larger than maximum valid length");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      textual_error.add_attachement_display("Message length", message_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_content_length());
      return Err(());
    }

    let message = &mut self.message_buffer[..message_length];

    if let Err(error) = self.stream.read_exact(message) {
      textual_error.add_message("An io error occured while reading the message");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      textual_error.add_attachement_display("Io error", error);
      return Err(());
    }

    let Ok(message) = deserialize(message, &mut textual_error) else {
      textual_error.add_message("An error occured while deserializing the message");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      return Err(());
    };

    Ok(message)
  }

  pub async fn send<T>(&mut self, content: &T, textual_error: &mut impl IsTextualError) -> Result<(), ()> 
  where 
    T: Serialize
  {
    let mut textual_error = textual_error.optional_context("Sending a message over a UnixStream");

    let content_buffer = &mut self.message_buffer[MESSAGE_LENGTH_SIZE..];

    let Ok(content_length) = serialize(
      content, 
      content_buffer,
      &mut textual_error,
    ) else {          
      // .with_message("Failed to serialize the message")
      // .with_attachement_display("Message data type name", type_name::<T>())
      // .with_attachement_display("Message buffer length", self.message_buffer.len())

      return Err(());
    };
    
    let Ok(content_length_as_u32): Result<u32, _> = content_length.try_into() else {
      return Err(());
    };

    let content_length_buffer = &mut self.message_buffer[0..MESSAGE_LENGTH_SIZE];
    content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

    let datagram = &self.message_buffer[
      ..
      MESSAGE_LENGTH_SIZE + content_length
    ];

    self.stream
      .write_all(&datagram)
      .await
      .map_err(|_| {
        ()
      })?;

    Ok(())
  }

  // pub async fn send_or_error<T>(&mut self, content: &T) -> Result<(), SendErrorCode> 
  // where 
  //   T: Serialize
  // {
  //   let content_buffer = &mut self.message_buffer[MESSAGE_LENGTH_SIZE..];

  //   let Ok(content_length) = serialize(content, content_buffer) else {
  //     return Err(SendErrorCode::ErrorWhileSerializingDatagramContent);
  //   };
    
  //   if content_length > self.maximum_content_length() {
  //     return Err(SendErrorCode::DatagramContentLengthLargerThanMaximumLength);
  //   }

  //   let Ok(content_length_as_u32): Result<u32, _> = content_length.try_into() else {
  //     return Err(SendErrorCode::DatagramContentLengthLargerThanMaximumLength);
  //   };

  //   let content_length_buffer = &mut self.message_buffer[0..MESSAGE_LENGTH_SIZE];
  //   content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

  //   let datagram = &self.message_buffer[
  //     ..
  //     MESSAGE_LENGTH_SIZE + content_length
  //   ];

  //   self.stream
  //     .write_all(&datagram)
  //     .await
  //     .map_err(|error| {
  //       SendErrorCode::IoErrorWhileWritingDatagram(error)
  //     })
  // }

  // pub async fn recv_or_error<T>(&mut self) -> Result<T, RecvErrorCode> 
  // where 
  //   T: DeserializeOwned
  // {
  //   let mut content_length = [0; MESSAGE_LENGTH_SIZE];

  //   self
  //     .stream
  //     .read_exact(&mut content_length)
  //     .await
  //     .map_err(|error| {
  //       RecvErrorCode::IoErrorWhileReadingDatagramContentLength(error)
  //     })?;
    
  //   let content_length = u32::from_be_bytes(content_length) as usize;
  //   if content_length > self.maximum_content_length() {
  //     return Err(RecvErrorCode::DatagramContentLengthLargerThanMaximumLength(content_length));
  //   }

  //   let message_content = &mut self.message_buffer[..content_length];

  //   self
  //     .stream
  //     .read_exact(message_content)
  //     .await
  //     .map_err(|error| {
  //       RecvErrorCode::IoErrorWhileReadingDatagramContent(error)
  //     })?;

  //   deserialize(message_content)
  //     .map_err(|_| {
  //       RecvErrorCode::ErrorWhileDeserializingDatagramContent
  //     })
  // }

  pub async fn send_or_textual_error<T>(&mut self, content: &T) -> Result<(), TextualError> 
  where 
    T: Serialize
  {
    let content_buffer = &mut self.message_buffer[MESSAGE_LENGTH_SIZE..];

    let content_length = serialize(content, content_buffer).map_err(|error| {
      error
        .with_context("Sending a message over a UnixStream")
        .with_message("Failed to serialize the message")
        .with_attachement_display("Message data type name", type_name::<T>())
        .with_attachement_display("Message buffer length", self.message_buffer.len())
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

    let content_length_buffer = &mut self.message_buffer[0..MESSAGE_LENGTH_SIZE];
    content_length_buffer.copy_from_slice(&content_length_as_u32.to_be_bytes());

    let datagram = &self.message_buffer[
      ..
      MESSAGE_LENGTH_SIZE + content_length
    ];

    self.stream
      .write_all(&datagram)
      .await
      .map_err(|error| {
        TextualError::new("Sending a message over a UnixStream")
          .with_message("An io error occured")
          .with_attachement_display("Message data type name", type_name::<T>())
          .with_attachement_display("Io error", error)

      })
  }

  // pub async fn recv_or_textual_error<T>(&mut self) -> Result<T, TextualError> 
  // where 
  //   T: DeserializeOwned
  // {
  //   let mut content_length = [0; MESSAGE_LENGTH_SIZE];

  //   self
  //     .stream
  //     .read_exact(&mut content_length)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Receiving a message over a UnixStream")
  //         .with_message("An io error occured while reading the message length")
  //         .with_attachement_display("Message data type", type_name::<T>())
  //         .with_attachement_display("Io error", error)
  //     })?;
    
  //   let content_length = u32::from_be_bytes(content_length) as usize;
  //   if content_length > self.maximum_content_length() {
  //     return Err(
  //       TextualError::new("Receiving a message over a UnixStream")
  //         .with_message("Message length is larger than maximum valid length")
  //         .with_attachement_display("Message data type", type_name::<T>())
  //         .with_attachement_display("Message length", content_length)
  //         .with_attachement_display("Message maximum valid length", self.maximum_content_length())
  //     );
  //   }

  //   let message_content = &mut self.message_buffer[..content_length];

  //   self
  //     .stream
  //     .read_exact(message_content)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Receiving a message over a UnixStream")
  //         .with_message("An io error occured while reading the message")
  //         .with_attachement_display("Message data type", type_name::<T>())
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   deserialize(message_content)
  //     .map_err(|error| {
  //       TextualError::new("Receiving a message over a UnixStream")
  //         .with_message("An error occured while deserializing the message")
  //         .with_attachement_display("Message data type", type_name::<T>())
  //         .with_attachement_display("Deserialization error", error)
  //     })
  // }
}
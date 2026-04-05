use std::any::type_name;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use crate::x::IsTextualError;
use super::{BufferLength, MessageLength, IsSerializable, IsSerializationFormat, IsDeserializable};

pub struct AsyncStream {
  stream: UnixStream,
  buffer: Vec<u8>,
}

impl AsyncStream {
  pub fn construct(
    stream: UnixStream, 
    maximum_buffer_length: BufferLength,
  ) -> Self {
    Self {
      stream,
      buffer: vec![0; maximum_buffer_length.get()],
    }
  }
  
  pub async fn connect(
    path: impl AsRef<Path>, 
    maximum_buffer_length: BufferLength,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    match UnixStream::connect(&path).await {
      Ok(stream) => {
        Ok(Self::construct(
          stream, 
          maximum_buffer_length,
        ))
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

  pub fn maximum_message_size(&self) -> usize {
    self.buffer.len() - MessageLength::BINARY_SIZE
  }

  pub async fn read<Message, SerializationFormat>(
    &mut self, 
    format: &SerializationFormat,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Message, ()> 
  where 
    Message: IsDeserializable,
    SerializationFormat: IsSerializationFormat,
  {
    let mut textual_error = textual_error
      .optional_context("Receiving a message over a UnixStream");

    let mut message_length = [0; MessageLength::BINARY_SIZE];

    self
      .stream
      .read_exact(&mut message_length)
      .await
      .map_err(|error| {
        textual_error.add_message("An io error occured while reading the message length");
        textual_error.add_attachement_display("Message data type", type_name::<Message>());
        textual_error.add_attachement_display("Io error", error);
      })?;
    
    let message_length = u32::from_be_bytes(message_length) as usize;
    if message_length > self.maximum_message_size() {
      textual_error.add_message("Message length is larger than maximum valid length");
      textual_error.add_attachement_display("Message data type", type_name::<Message>());
      textual_error.add_attachement_display("Message length", message_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_size());
      return Err(());
    }

    let message = &mut self.buffer[..message_length];

    self
      .stream
      .read_exact(message)
      .await
      .map_err(|error| {
        textual_error.add_message("An io error occured while reading the message");
        textual_error.add_attachement_display("Message data type", type_name::<Message>());
        textual_error.add_attachement_display("Io error", error);
      })?;

    format
      .deserialize(
        message, 
        &mut textual_error,
      )
      .map_err(|_| {
        textual_error.add_message("An error occured while deserializing the message");
        textual_error.add_attachement_display("Message data type", type_name::<Message>());
      })
  }

  pub async fn write<Message, SerializationFormat>(
    &mut self, 
    message: &Message, 
    format: &SerializationFormat,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> 
  where 
    Message: IsSerializable,
    SerializationFormat: IsSerializationFormat,
  {
    let mut textual_error = textual_error
      .optional_context("Sending a message over a UnixStream");

    let message_buffer = &mut self.buffer[MessageLength::BINARY_SIZE..];

    let message_length = format
      .serialize(
        message, 
        message_buffer,
        &mut textual_error,
      )
      .map_err(|_| {
        textual_error.add_message("Failed to serialize the message");
        textual_error.add_attachement_display("Message data type name", type_name::<Message>());
        textual_error.add_attachement_display("Message buffer length", self.buffer.len());
      })?;
    
    if message_length > self.maximum_message_size() {
      textual_error.add_message("Message length is larger than the maximum allowed length");
      textual_error.add_attachement_display("Message data type name", type_name::<Message>());
      textual_error.add_attachement_display("Message length", message_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_size());
      return Err(());
    }

    let message_length_as_u32: u32 = message_length
      .try_into()
      .map_err(|_| {
        textual_error.add_message("Message length is larger than the maximum allowed length");
        textual_error.add_attachement_display("Message data type name", type_name::<Message>());
        textual_error.add_attachement_display("Message length", message_length);
        textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_size());
      })?;

    let message_length_buffer = &mut self.buffer[0..MessageLength::BINARY_SIZE];
    message_length_buffer.copy_from_slice(&message_length_as_u32.to_be_bytes());

    let length_and_message = &self.buffer[
      ..
      MessageLength::BINARY_SIZE + message_length
    ];

    self
      .stream
      .write_all(&length_and_message) 
      .await
      .map_err(|error| {
        textual_error.add_message("An io error occured");
        textual_error.add_attachement_display("Message data type name", type_name::<Message>());
        textual_error.add_attachement_display("Io error", error);
      })
  }
}
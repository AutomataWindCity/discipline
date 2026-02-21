use std::{any::type_name, io::{Read, Write}, path::Path};
use serde::{Serialize, de::DeserializeOwned};
use std::os::unix::net::UnixStream;
use crate::x::IsTextualError;
use super::{deserialize, serialize};

const MESSAGE_CONTENT_LENGTH_SIZE: usize = size_of::<u32>();

pub struct Stream {
  stream: UnixStream,
  message_buffer: Vec<u8>,
}

impl Stream {
  pub fn construct(stream: UnixStream, maximum_message_content_length: usize) -> Self {
    Self {
      stream,
      message_buffer: vec![0; MESSAGE_CONTENT_LENGTH_SIZE + maximum_message_content_length],
    }
  }
  
  pub fn connect(
    path: impl AsRef<Path>, 
    maximum_message_content_length: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Stream, ()> {
    match UnixStream::connect(&path) {
      Ok(stream) => {
        Ok(Stream::construct(
          stream, 
          maximum_message_content_length,
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

  fn maximum_message_content_length(&self) -> usize {
    self.message_buffer.len() - MESSAGE_CONTENT_LENGTH_SIZE
  }

  pub fn recv<T>(
    &mut self, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<T, ()> 
  where 
    T: DeserializeOwned
  {
    let mut textual_error = textual_error.optional_context("Receiving a message over a UnixStream");

    let mut message_content_length = [0; MESSAGE_CONTENT_LENGTH_SIZE];

    if let Err(error) = self.stream.read_exact(&mut message_content_length) {
      textual_error.add_message("An io error occured while reading the message length");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      textual_error.add_attachement_display("Io error", error);
      return Err(());
    }
    
    let message_length = u32::from_be_bytes(message_content_length) as usize;
    if message_length > self.maximum_message_content_length() {
      textual_error.add_message("Message length is larger than maximum valid length");
      textual_error.add_attachement_display("Message data type", type_name::<T>());
      textual_error.add_attachement_display("Message length", message_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_content_length());
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

  pub fn send<T>(&mut self, content: &T, textual_error: &mut impl IsTextualError) -> Result<(), ()> 
  where 
    T: Serialize
  {
    let mut textual_error = textual_error.optional_context("Sending a message over a UnixStream");

    let message_content_buffer = &mut self.message_buffer[MESSAGE_CONTENT_LENGTH_SIZE..];

    let Ok(message_content_length) = serialize(
      content, 
      message_content_buffer,
      &mut textual_error,
    ) else {          
      textual_error.add_message("Failed to serialize the message");
      textual_error.add_attachement_display("Message data type name", type_name::<T>());
      textual_error.add_attachement_display("Message buffer length", self.message_buffer.len());
      return Err(());
    };
    
    if message_content_length > self.maximum_message_content_length() {
      textual_error.add_message("Message length is larger than the maximum allowed length");
      textual_error.add_attachement_display("Message data type name", type_name::<T>());
      textual_error.add_attachement_display("Message length", message_content_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_content_length());
      return Err(());
    }

    let Ok(message_content_length_as_u32): Result<u32, _> = message_content_length.try_into() else {
      textual_error.add_message("Message length is larger than the maximum allowed length");
      textual_error.add_attachement_display("Message data type name", type_name::<T>());
      textual_error.add_attachement_display("Message length", message_content_length);
      textual_error.add_attachement_display("Message maximum valid length", self.maximum_message_content_length());
      return Err(());
    };

    let message_content_length_buffer = &mut self.message_buffer[0..MESSAGE_CONTENT_LENGTH_SIZE];
    message_content_length_buffer.copy_from_slice(&message_content_length_as_u32.to_be_bytes());

    let message = &self.message_buffer[
      ..
      MESSAGE_CONTENT_LENGTH_SIZE + message_content_length
    ];

    if let Err(error) = self.stream.write_all(&message) {
      textual_error.add_message("An io error occured");
      textual_error.add_attachement_display("Message data type name", type_name::<T>());
      textual_error.add_attachement_display("Io error", error);
    }

    Ok(())
  }

}
use std::path::Path;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use super::{Serializable, Deserializable};

const DATAGRAM_CONTENT_LENGTH_SIZE: usize = size_of::<u32>();
const MAXIMUM_DATAGRAM_CONTENT_LENGTH: usize = 876876;

pub struct DatagramConnection {
  connection: UnixStream,
  datagram_content_buffer: Vec<u8>,
}

impl DatagramConnection {
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

  pub fn construct(connection: UnixStream, maximum_length: usize) -> Self {
    Self {
      connection,
      datagram_content_buffer: vec![0; maximum_length],
    }
  }

  pub async fn recv<T>(&mut self) -> Result<T, ()> 
  where 
    T: Deserializable
  {
    let mut content_length = [0; DATAGRAM_CONTENT_LENGTH_SIZE];

    if let Err(_) = self.connection.read_exact(&mut content_length).await {
      return Err(());
    }
    
    let content_length = u32::from_be_bytes(content_length) as usize;
    if content_length > MAXIMUM_DATAGRAM_CONTENT_LENGTH {
      return Err(());
    }

    let message_content = &mut self.datagram_content_buffer[..content_length];

    if let Err(_) = self.connection.read_exact(message_content).await {
      return Err(());
    }

    T::deserialize(message_content).map_err(|_| ())
  }

  pub async fn send<T>(&mut self, content: &T) -> Result<(), ()> 
  where 
    T: Serializable
  {
    let content_buffer = &mut self.datagram_content_buffer[DATAGRAM_CONTENT_LENGTH_SIZE..];

    let Ok(content_length) = T::serialize(content, content_buffer) else {
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
}

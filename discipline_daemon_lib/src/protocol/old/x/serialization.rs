use std::any::type_name;
use serde::{Serialize, de::DeserializeOwned};
use crate::TextualError;

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
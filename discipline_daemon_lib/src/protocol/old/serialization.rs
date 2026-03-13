use std::any::type_name;
use serde::{Serialize, de::DeserializeOwned};
use crate::IsTextualError;

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

pub fn serialize<T>(value: &T, destination: &mut [u8], textual_error: &mut impl IsTextualError) -> Result<usize, ()> 
where 
  T: Serialize
{
  match bincode::serde::encode_into_slice(value, destination, BINCODE_CONFIG) {
    Ok(value) => {
      Ok(value)
    }
    Err(error) => {
      textual_error.change_context("Serializing a value using bincode");
      textual_error.add_message("Bincode failed to serialize value");
      textual_error.add_attachement_display("Value type name", type_name::<T>());
      textual_error.add_attachement_display("Bincode error", error);
      return Err(());
    }
  }
}

pub fn deserialize<T>(slice: &[u8], textual_error: &mut impl IsTextualError) -> Result<T, ()>
where 
  T: DeserializeOwned
{
  let mut textual_error = textual_error.optional_context(format!("Deserializing byte array as {} using bincode", type_name::<T>()));
  
  let (value, read_bytes) = match bincode::serde::decode_from_slice(slice, BINCODE_CONFIG) {
    Ok(value) => {
      value
    }
    Err(error) => {
      textual_error.add_attachement_display("Error", error);
      return Err(());
    }
  };

  if read_bytes != slice.len() {
    textual_error.with_message(format!("Bincode deserialized the byte array successfully, but the number of bytes bincode read is not the same as the byte array length, which shouldn't be possible since the byte array is expected to be the binary repreentation of {}, without additional or missing bytes.", type_name::<T>()));
    textual_error.with_attachement_display("Byte array length", slice.len());
    textual_error.with_attachement_debug("Byte array", slice);
    return Err(());
  }
  
  Ok(value)
}
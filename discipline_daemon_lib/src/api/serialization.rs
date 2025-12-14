use std::any::type_name;
use serde::{Serialize, de::DeserializeOwned};
use crate::x::{TextualError, };

static PREFACE_MAGIC_BYTES: [u8; 16] = [
  0xDE, 0xAD, 0xBE, 0xEF,
  0xCA, 0xFE, 0xBA, 0xBE,
  0x0D, 0x15, 0xEA, 0x5E,
  0x50, 0x52, 0x30, 0x54
];

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

pub fn bincode_serialize<T>(value: &T) -> Result<Vec<u8>, TextualError> 
where 
  T: Serialize
{
  bincode::serde::encode_to_vec(value, BINCODE_CONFIG)
    .map_err(|error| {
      TextualError::new(format!("Serializing {} using bincode", type_name::<T>()))
        .with_attachement_display("Error", error)
    })
}

pub fn bincode_deserialize<T>(slice: &[u8]) -> Result<T, TextualError>
where 
  T: DeserializeOwned
{
  let (value, read_bytes) = match bincode::serde::decode_from_slice(slice, BINCODE_CONFIG) {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(
        TextualError::new(format!("Deserializing byte array as {} using bincode", type_name::<T>()))
          .with_attachement_display("Error", error)
      );
    }
  };

  if read_bytes != slice.len() {
    return Err(
      TextualError::new(format!("Deserializing byte array as {} using bincode", type_name::<T>()))
        .with_message(format!("Bincode deserialized the byte array successfully, but the number of bytes bincode read is not the same as the byte array length, which shouldn't be possible since the byte array is expected to be the binary repreentation of {}, without additional or missing bytes.", type_name::<T>()))
        .with_attachement_display("Byte array length", slice.len())
        .with_attachement_debug("Byte array", slice)
    )
  }
  
  Ok(value)
}
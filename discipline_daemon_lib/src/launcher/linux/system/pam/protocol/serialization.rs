use crate::launcher::linux::pam::stream::{IsSerializationFormat, IsDeserializable, IsSerializable};

pub struct BincodeSerializationFormat {}

impl IsSerializationFormat for BincodeSerializationFormat {
  fn deserialize<T>(
    &self, 
    buffer: &[u8], 
    textual_error: &mut impl IsTextualError,
  ) -> Result<T, ()>
  where
    T: IsDeserializable 
  {
    todo!()
  }

  fn serialize(
    &self, 
    value: &impl IsSerializable, 
    buffer: &mut [u8],
    textual_error: &mut impl IsTextualError,
  ) -> Result<usize, ()> 
  {
    todo!()
  }
}

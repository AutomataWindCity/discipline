pub trait Serializable {
  type Error;

  fn serialize(value: &Self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait Deserializable: Sized {
  type Error;

  fn deserialize(buffer: &mut [u8]) -> Result<Self, Self::Error>;
}
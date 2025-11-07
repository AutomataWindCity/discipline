pub use uuid::Bytes;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UuidV4 {
  uuid: uuid::Uuid,
}

impl UuidV4 {
  pub fn generate() -> Self {
    Self {
      uuid: uuid::Uuid::new_v4(),
    }
  }
}

mod serialization {
  use crate::x::{TextualError, UuidV4};
  use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
  use uuid::Uuid;

  impl Serialize for UuidV4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      self.uuid.as_bytes().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for UuidV4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'a>,
    {
      let bytes = Vec::<u8>::deserialize(deserializer).map_err(|error| {
        Error::custom(
          TextualError::new("Deserializing Vec<u8>")
            .with_attachement_display("Error", error)
            .with_context("Deserializing UuidV4 which is serialized as Vec<u8>"),
        )
      })?;

      let uuid = Uuid::from_slice(&bytes).map_err(|error| {
        Error::custom(
          TextualError::new("Creating UuidV4 from Vec<u8>")
            .with_attachement_display("Error", error)
            .with_context("Deserializing UuidV4, which is serialized as Vec<u8>")
        )
      })?;

      // `Version::Random` refers to version v4, which is what we are expecting.
      if uuid.get_version_num() != 4 {
        return Err(Error::custom(
          TextualError::new("Creating UuidV4 from Vec<u8>")
            .with_message("Vec<u8> contains a valid Uuid, but its version number is not 4, also known as Random")
            .with_attachement_display("Found Uuid version number", uuid.get_version_num())
            .with_context("Deserializing UuidV4, which is serialized as Vec<u8>")
        ));
      }

      Ok(UuidV4 { uuid })
    }
  }
}

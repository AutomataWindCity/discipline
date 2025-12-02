use std::str::FromStr;
pub use uuid::{Bytes, Uuid, Error};
use crate::x::{ToTextualError, TextualErrorContext};

pub enum CreateFromStringError {
  X(Error)
}

impl ToTextualError for CreateFromStringError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new("Creating UuidV4 from string");

    match self {
      Self::X(error) => {
        context.add_message("String is malformed");
        context.add_attachement_display("Parsing error", error);
      }
    }

    context
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UuidV4 {
  inner: Uuid,
}

impl UuidV4 {
  pub fn generate() -> Self {
    Self {
      inner: Uuid::new_v4(),
    }
  }

  pub fn from_string(string: &str) -> Result<Self, CreateFromStringError> {
    match Uuid::from_str(string) {
      Ok(inner) => {
        Ok(UuidV4 { inner })
      }
      Err(error) => {
        Err(CreateFromStringError::X(error))
      }
    }
  }

  pub fn to_string(&self) -> String {
    self.inner.to_string()
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
      self.inner.as_bytes().serialize(serializer)
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

      Ok(UuidV4 { inner: uuid })
    }
  }
}
#[derive(Debug, Clone)]
pub struct UserName {
  inner: String,
}

#[derive(Debug, Clone)]
pub enum CreateFromStringError {
  MinimumLengthViolated { string: String },
  MaximumLengthViolated { string: String },
}

impl UserName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<UserName, CreateFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Err(CreateFromStringError::MinimumLengthViolated { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Err(CreateFromStringError::MaximumLengthViolated { string });
    }
    Ok(Self { inner: string })
  }

  pub fn as_str(&self) -> &str {
    &self.inner
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::TextualError;
  use crate::x::user_name::{CreateFromStringError, UserName};

  impl Serialize for UserName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.as_str().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for UserName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      let string = String::deserialize(deserializer).map_err(|error| {
        Error::custom(
          TextualError::new("Deserializing String")
            .with_attachement_display("Error", error)
            .with_context("Deserializing UserName which is serialized as String")
        )
      })?;

      UserName::new(string).map_err(|error| match error {
        CreateFromStringError::MaximumLengthViolated { string } => {
          Error::custom(TextualError::new("Creating UserName from String")
            .with_message("String is too long")
            .with_attachement_display("Maximum Length", UserName::MAXIMUM_LENGTH)
            .with_attachement_display("Minimum Length", UserName::MINIMUM_LENGTH)
            .with_attachement_display("String Length", string.len())
            .with_attachement_display("String", string)
            .with_context("Deserializing UserName which is serialized as String")
          )
        }
        CreateFromStringError::MinimumLengthViolated { string } => {
          Error::custom(TextualError::new("Creating UserName from String")
            .with_message("String is too short")
            .with_attachement_display("Maximum Length", UserName::MAXIMUM_LENGTH)
            .with_attachement_display("Minimum Length", UserName::MINIMUM_LENGTH)
            .with_attachement_display("String Length", string.len())
            .with_attachement_display("String", string)
            .with_context("Deserializing UserName which is serialized as String")
          )
        }
      })
    }
  }
}
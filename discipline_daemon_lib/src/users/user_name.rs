use std::any::type_name;
use crate::x::{ToTextualError, TextualErrorContext};

#[derive(Debug, Clone)]
pub struct UserName {
  inner: String,
}

#[derive(Debug, Clone)]
pub enum CreateFromStringError {
  LengthViolation { string: String },
}

impl ToTextualError for CreateFromStringError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new(format!("Creating {} from String", type_name::<UserName>()));
   
    match self {
      Self::LengthViolation { string } => {
        context.add_message("String length is invalid");
        context.add_attachement_display("Minimum valid length", UserName::MINIMUM_LENGTH);
        context.add_attachement_display("Maximum valid length", UserName::MAXIMUM_LENGTH);
        context.add_attachement_display("Found string length", string.len());
        context.add_attachement_display("String", string);
      }
    }

    context
  }
}

impl UserName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<UserName, CreateFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Err(CreateFromStringError::LengthViolation { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Err(CreateFromStringError::LengthViolation { string });
    }
    Ok(Self { inner: string })
  }

  pub fn as_str(&self) -> &str {
    &self.inner
  }
  pub fn as_string(&self) -> &String {
    &self.inner
  }
}

mod serialization {
  use serde::{Serialize, Deserialize, de::Error};
  use crate::x::{TextualError, ToTextualError};
  use crate::x::user_name::UserName;

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

      UserName::new(string).map_err(|error| {
        Error::custom(
          error
            .to_textual_error()
            .with_context("Deserializing UserName which is serialized as String")
        )
      })
    }
  }
}
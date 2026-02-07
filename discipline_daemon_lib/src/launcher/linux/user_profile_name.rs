use std::any::type_name;
use crate::x::{ToTextualError, TextualErrorContext};

#[derive(Debug, Clone)]
pub struct UserProfileName {
  inner: String,
}

#[derive(Debug, Clone)]
pub enum CreateFromStringError {
  LengthViolation { string: String },
}

impl ToTextualError for CreateFromStringError {
  fn to_textual_error_context(&self) -> TextualErrorContext {
    let mut context = TextualErrorContext::new(format!("Creating {} from String", type_name::<UserProfileName>()));
   
    match self {
      Self::LengthViolation { string } => {
        context.add_message("String length is invalid");
        context.add_attachement_display("Minimum valid length", UserProfileName::MINIMUM_LENGTH);
        context.add_attachement_display("Maximum valid length", UserProfileName::MAXIMUM_LENGTH);
        context.add_attachement_display("Found string length", string.len());
        context.add_attachement_display("String", string);
      }
    }

    context
  }
}

impl UserProfileName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<UserProfileName, CreateFromStringError> {
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

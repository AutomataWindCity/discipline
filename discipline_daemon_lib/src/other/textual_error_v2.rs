use std::fmt::{Debug, Display};

pub trait TextualErrorV2 {
  fn context(&mut self, action: impl Into<String>) -> impl TextualErrorContextV2;
}

pub trait TextualErrorContextV2: TextualErrorV2 {
  fn write_message(&mut self, message: impl Into<String>);
  fn write_attachement(&mut self, name: impl Into<String>, value: impl Display);
  fn write_debug_attachement(&mut self, name: impl Into<String>, value: impl Debug);
  fn context(&mut self, action: impl Into<String>) -> impl TextualErrorContextV2;
}

pub struct TextualError {

}

pub struct TextualErrorContext {
  action: String,
  messages: Vec<String>,
  attachements: Vec<TextualErrorAttachement>,
}

pub struct TextualErrorAttachement {
  name: String,
  value: String,
}
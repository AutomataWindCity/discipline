use std::{fmt::{Debug, Display}, mem::replace};

pub struct TextualErrorAttachement {
  name: String,
  value: String,
}

impl TextualErrorAttachement {
  pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      value: value.into(),
    }
  }
}

pub struct TextualErrorContext {
  action: String,
  messages: Vec<String>,
  attachements: Vec<TextualErrorAttachement>,
}

pub struct TextualError {
  context: TextualErrorContext,
  eariler_contexts: Vec<TextualErrorContext>
}

impl TextualError {
  pub fn new(action: impl Into<String>) -> Self {
    Self {
      context: TextualErrorContext { 
        action: action.into(), 
        messages: Vec::new(), 
        attachements: Vec::new(),
      },
      eariler_contexts: Vec::new(),
    }
  }

  pub fn add_message(&mut self, new_error_message: impl Into<String>) {
    self.context.messages.push(new_error_message.into());
  }

  pub fn add_attachement_debug(&mut self, name: impl Into<String>, value: impl Debug) {
    self.context.attachements.push(TextualErrorAttachement {
      name: name.into(),
      value: format!("{value:?}"),
    });
  }

  pub fn add_attachement_display(&mut self, name: impl Into<String>, value: impl Display) {
    self.context.attachements.push(TextualErrorAttachement {
      name: name.into(),
      value: format!("{value}"),
    });
  }

  pub fn change_context(&mut self, new_context_action: impl Into<String>) {
    self.eariler_contexts.push(replace(&mut self.context, TextualErrorContext { 
      action: new_context_action.into(), 
      messages: Vec::new(),
      attachements: Vec::new(),
    }));
  }

  pub fn with_message(mut self, message: impl Into<String>) -> TextualError {
    self.add_message(message);
    self
  }

  pub fn with_attachement_debug(mut self, name: impl Into<String>, value: impl Debug) -> TextualError {
    self.add_attachement_debug(name, value);
    self
  }

  pub fn with_attachement_display(mut self, name: impl Into<String>, value: impl Display) -> TextualError {
    self.add_attachement_display(name, value);
    self
  }

  pub fn with_context(mut self, action: impl Into<String>) -> TextualError {
    self.change_context(action);
    self
  }
}

impl Display for TextualError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
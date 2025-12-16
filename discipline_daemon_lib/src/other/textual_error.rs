use std::mem::replace;
use std::fmt::{Debug, Display, Formatter, self};

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

impl TextualErrorContext {
  pub fn new(action: impl Into<String>) -> Self {
    Self {
      action: action.into(),
      messages: Vec::new(),
      attachements: Vec::new(),
    }
  }

  pub fn add_message(&mut self, new_error_message: impl Into<String>) {
    self.messages.push(new_error_message.into());
  }

  pub fn add_attachement_debug(&mut self, name: impl Into<String>, value: impl Debug) {
    self.attachements.push(TextualErrorAttachement {
      name: name.into(),
      value: format!("{value:?}"),
    });
  }

  pub fn add_attachement_display(&mut self, name: impl Into<String>, value: impl Display) {
    self.attachements.push(TextualErrorAttachement {
      name: name.into(),
      value: format!("{value}"),
    });
  }

  pub fn with_message(mut self, message: impl Into<String>) -> Self {
    self.add_message(message);
    self
  }

  pub fn with_attachement_debug(mut self, name: impl Into<String>, value: impl Debug) -> Self {
    self.add_attachement_debug(name, value);
    self
  }

  pub fn with_attachement_display(mut self, name: impl Into<String>, value: impl Display) -> Self {
    self.add_attachement_display(name, value);
    self
  }
}

pub struct TextualError {
  context: TextualErrorContext,
  eariler_contexts: Vec<TextualErrorContext>,
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
    self.eariler_contexts.push(replace(
      &mut self.context,
      TextualErrorContext {
        action: new_context_action.into(),
        messages: Vec::new(),
        attachements: Vec::new(),
      },
    ));
  }

  pub fn with_message(mut self, message: impl Into<String>) -> TextualError {
    self.add_message(message);
    self
  }

  pub fn with_attachement_debug(
    mut self,
    name: impl Into<String>,
    value: impl Debug,
  ) -> TextualError {
    self.add_attachement_debug(name, value);
    self
  }

  pub fn with_attachement_display(
    mut self,
    name: impl Into<String>,
    value: impl Display,
  ) -> TextualError {
    self.add_attachement_display(name, value);
    self
  }

  pub fn with_context(mut self, action: impl Into<String>) -> TextualError {
    self.change_context(action);
    self
  }
}

pub trait ToTextualError {
  fn to_textual_error_context(&self) -> TextualErrorContext;

  fn to_textual_error(&self) -> TextualError {
    TextualError {
      eariler_contexts: Vec::new(),
      context: self.to_textual_error_context(),
    }
  }
}

// TODO: Make sure these implementations are correct
impl Display for TextualErrorAttachement {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "    • {}: {}", self.name, self.value)
  }
}

impl Display for TextualErrorContext {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "↪ {} ⭐", self.action)?;

    if !self.messages.is_empty() {
      writeln!(f, "")?;
      for msg in &self.messages {
        writeln!(f, "    • {}  📣", msg)?;
      }
    }

    if !self.attachements.is_empty() {
      writeln!(f, "")?;
      for att in &self.attachements {
        writeln!(f, "{att}")?;
      }
    }

    Ok(())
  }
}

impl Display for TextualError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // earlier contexts printed first, oldest to newest
    for ctx in &self.eariler_contexts {
      writeln!(f, "{ctx}")?;
      writeln!(f)?;
    }

    // final (current) context
    write!(f, "{}", self.context)
  }
}

impl Debug for TextualError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

pub trait IsTextualError {
  fn new(action: impl Into<String>) -> Self;
  fn add_message(&mut self, new_error_message: impl Into<String>);
  fn add_attachement_debug(&mut self, name: impl Into<String>, value: impl Debug);
  fn add_attachement_display(&mut self, name: impl Into<String>, value: impl Display);
  fn change_context(&mut self, new_context_action: impl Into<String>);
  fn with_message(self, message: impl Into<String>) -> Self;
  fn with_attachement_debug(self, name: impl Into<String>, value: impl Debug) -> TextualError;
  fn with_attachement_display(self, name: impl Into<String>, value: impl Display) -> TextualError;
  fn with_context(self, action: impl Into<String>) -> TextualError;
}
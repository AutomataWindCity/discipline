use std::collections::HashMap;

pub struct Element {
  name: String,
  attributes: HashMap<String, String>,
  children: Vec<Element>,
}

fn element(name: impl Into<String>) -> Element {
  Element { 
    name: name.into(), 
    attributes: HashMap::new(), 
    children: Vec::new(),
  }
}

impl Element {
  pub fn with_child(mut self, child: Element) -> Self {
    self.children.push(child);
    self
  }
  
  pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
    self.attributes.insert(name.into(), value.into());
    self
  }

  pub fn with_text(&mut self, text: impl Into<String>) -> Self {
    todo!()
  }
}

fn moon() {
  let x = element("div")
    .with_child(element("p").with_text("User Profiles"));
}
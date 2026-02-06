use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAccessRegulation {}

impl DeviceAccessRegulation {
  pub fn new() -> Self {
    Self {

    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenAccessRegulation {}

impl ScreenAccessRegulation {
  pub fn new() -> Self {
    Self {
      
    }
  }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetAccessRegulation {}

impl InternetAccessRegulation {
  pub fn new() -> Self {
    Self {
      
    }
  }
}
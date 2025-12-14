use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, CountdownAfterPleaConditional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultName {
  pub(super) string: String,
}

#[derive(Debug, Clone)]
pub enum CreateVaultNameFromStringError {
  LengthViolation { string: String }
}

impl VaultName {
  pub(super) const MINIMUM_LENGTH: usize = 1;
  pub(super) const MAXIMUM_LENGTH: usize = 300;

  pub fn new(string: String) -> Result<Self, CreateVaultNameFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Ok(Self { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Ok(Self { string });
    }
    Ok(Self { string })
  }
}

impl AsRef<str> for VaultName {
  fn as_ref(&self) -> &str {
    &self.string
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Datum {
  pub(super) string: String,
}

#[derive(Debug, Clone)]
pub enum CreateDatumFromStringError {
  LengthViolation { string: String }
}

impl Datum {
  pub(super) const MINIMUM_LENGTH: usize = 1;
  pub(super) const MAXIMUM_LENGTH: usize = 10000;

  pub fn new(string: String) -> Result<Self, CreateDatumFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Ok(Self { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Ok(Self { string });
    }
    Ok(Self { string })
  }
}

impl AsRef<str> for Datum {
  fn as_ref(&self) -> &str {
    &self.string
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VaultProtector {
  CountdownAfterPlea(CountdownAfterPleaConditional)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
  name: VaultName,
  protector: VaultProtector,
}

// TODO: Rename to CommonInfo or Singleton
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultsSingleton {
  vault_number: usize,
  maximum_vault_number: usize,
  data_number: usize,
  maximum_data_number: usize,
}

impl Default for VaultsSingleton {
  fn default() -> Self {
    Self {
      data_number: 0,
      maximum_data_number: 500,
      vault_number: 0,
      maximum_vault_number: 500,
    }
  }
}

impl VaultsSingleton {
  pub fn construct(
    vault_number: usize,
    maximum_vault_number: usize,
    data_number: usize,
    maximum_data_number: usize,
  ) -> Self {
    Self {
      vault_number,
      maximum_vault_number,
      data_number,
      maximum_data_number,
    }
  }
  pub fn try_decrement_vault_number(&mut self) {
    if let None = self.vault_number.checked_sub(1) {
      // TODO: Log this case
    }
  }
  
  pub fn try_increment_vault_number(&mut self) {
    if let None = self.vault_number.checked_add(1) {
      // TODO: Log this case
    }
  }
  
  pub fn may_add_another_vault(&self) -> bool {
    self.vault_number < self.maximum_vault_number
  }

  pub fn try_decrement_data_number(&mut self) {
    if let None = self.data_number.checked_sub(1) {
      // TODO: Log this case
    }
  }

  pub fn try_increment_data_number(&mut self) {
    if let None = self.data_number.checked_add(1) {
      // TODO: Log this case
    }
  }

  pub fn may_add_another_datum(&mut self) -> bool {
    self.data_number < self.maximum_data_number
  }

  pub fn get_vault_number(&self) -> usize {
    self.vault_number
  }
  pub fn get_maximum_vault_number(&self) -> usize {
    self.maximum_vault_number
  }
  pub fn get_data_number(&self) -> usize {
    self.data_number
  }
  pub fn get_maximum_data_number(&self) -> usize {
    self.maximum_data_number
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
  User { user_id: UuidV4 }
}
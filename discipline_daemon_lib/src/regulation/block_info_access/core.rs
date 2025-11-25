
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossVaultGroupInfo {
  vault_number: usize,
  maximum_vault_number: usize,
  data_number: usize,
  maximum_data_number: usize,
}

impl CrossVaultGroupInfo {
  pub fn try_decrement_vault_number(&mut self) {}
  pub fn try_increment_vault_number(&mut self) {}
  pub fn may_add_another_vault(&self) -> bool {
    todo!()
  }
  pub fn try_decrement_data_number(&mut self) {}
  pub fn try_increment_data_number(&mut self) {}
  pub fn may_add_another_datum(&mut self) -> bool {
    todo!()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
  User { user_id: UuidV4 }
}
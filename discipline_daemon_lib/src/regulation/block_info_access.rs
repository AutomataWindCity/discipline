use serde::{Serialize, Deserialize};
use crate::x::{UuidV4, CountdownAfterPleaConditional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultName {
  string: String,
}

#[derive(Debug, Clone)]
pub enum CreateVaultNameFromStringError {
  LengthViolation { string: String }
}

impl VaultName {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 300;

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
pub struct VaultDatum {
  string: String,
}

#[derive(Debug, Clone)]
pub enum CreateVaultDatumFromStringError {
  LengthViolation { string: String }
}

impl VaultDatum {
  const MINIMUM_LENGTH: usize = 1;
  const MAXIMUM_LENGTH: usize = 10000;

  pub fn new(string: String) -> Result<Self, CreateVaultDatumFromStringError> {
    if string.len() < Self::MINIMUM_LENGTH {
      return Ok(Self { string });
    }
    if string.len() > Self::MAXIMUM_LENGTH {
      return Ok(Self { string });
    }
    Ok(Self { string })
  }
}

impl AsRef<str> for VaultDatum {
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

mod serialization {
  use serde::{Serialize, Serializer, Deserializer, Deserialize, de::Error};
  use crate::x::TextualError;
  use super::{VaultName, VaultDatum, CreateVaultDatumFromStringError, CreateVaultNameFromStringError};

  impl Serialize for VaultName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer 
    {
      serializer.serialize_str(&self.string)
    }
  }

  impl<'a> Deserialize<'a> for VaultName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'a> 
    {
      let string = String::deserialize(deserializer).map_err(|error| {
        Error::custom(
          TextualError::new("Deserializing String")
            .with_message("Failed to deserialize a value of type Srting")
            .with_attachement_display("Error", error)
            .with_context("Deserializing VaultName, which is serialized as String")
        )
      })?;

      Self::new(string).map_err(|error| Error::custom(match error {
        CreateVaultNameFromStringError::LengthViolation { .. } => {
          TextualError::new("Creating VaultName from String")
            .with_message("String length is invalid")
            .with_attachement_display("Minimum length", VaultName::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", VaultName::MAXIMUM_LENGTH)
            .with_context("Deserializing VaultName")
        }
      }))
    }
  }

  impl Serialize for VaultDatum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer 
    {
      serializer.serialize_str(&self.string)
    }
  }

  impl<'a> Deserialize<'a> for VaultDatum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'a> 
    {
      let string = String::deserialize(deserializer).map_err(|error| {
        Error::custom(
          TextualError::new("Deserializing String")
            .with_message("Failed to deserialize a value of type Srting")
            .with_attachement_display("Error", error)
            .with_context("Deserializing VaultDatum, which is serialized as String")
        )
      })?;

      Self::new(string).map_err(|error| Error::custom(match error {
        CreateVaultDatumFromStringError::LengthViolation { .. } => {
          TextualError::new("Creating VaultDatum from String")
            .with_message("String length is invalid")
            .with_attachement_display("Minimum length", VaultDatum::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", VaultDatum::MAXIMUM_LENGTH)
            .with_context("Deserializing VaultDatum")
        }
      }))
    }
  }
}

pub mod database {
  use crate::x::{UuidV4, Database, TextualError, conditionals, database::*};
  use super::*;

  pub enum ProtectorType {
    CountdownAfterPlea,
  }

  impl ProtectorType {
    const COUNTDOWN_AFTER_PLEA_AS_NUMBER: u8 = 0;

    pub fn from_number(number: u8) -> Result<Self, TextualError> {
      match number {
        Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER => {
          Ok(Self::CountdownAfterPlea)
        }
        other => {
          Err(
            TextualError::new("Creating VaultProtectorType from variant number")
              .with_message(format!("Unknown variant. Expected {} (for CountdownAfterPlea) but found {}", Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER, other))
          )
        }
      }
    }

    pub fn to_number(&self) -> u8 {
      match self {
        Self::CountdownAfterPlea => {
          Self::COUNTDOWN_AFTER_PLEA_AS_NUMBER
        }
      }
    }
  }

  impl WriteScalarValue for ProtectorType {
    fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
      writer.write_scalar_value(&value.to_number());
    }
  }

  impl ReadScalarValue for ProtectorType {
    fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
      reader.read_scalar_value().and_then(Self::from_number)
    }
  }

  pub struct ProtectorSchema {
    enum_type: Key,
    enum_countdown_after_plea: conditionals
      ::countdown_after_plea
      ::database
      ::Schema,
  }

  impl WriteCompoundValue for VaultProtector {
    type Schema = ProtectorSchema;

    fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
      match value {
        VaultProtector::CountdownAfterPlea(inner) => {
          writer.write_scalar_value(schema.enum_type, &ProtectorType::CountdownAfterPlea);
          writer.write_compound_value(&schema.enum_countdown_after_plea, inner);
        }
      }
    }
  }

  pub struct VaultCollectionSchema {
    id: Key,
    name: Key,
    protector: ProtectorSchema,
  }

  pub enum AddVaultError {
    DuplicateId,
    Other,
  }

  pub async fn add_vault(
    location: &Location,
    database: &Database,
    vault_id: &UuidV4,
    vault_name: &VaultName,
    vault_protector: &VaultProtector,
  ) -> Result<(), AddVaultError> {
    todo!()
  }

  pub enum DeleteVaultError {
    NoSuchVault,
    Other,
  }

  pub async fn delete_vault(
    location: &Location,
    database: &Database,
    vault_id: &UuidV4,
  ) -> Result<(), DeleteVaultError> {
    todo!()
  }

  pub enum ChangeVaultNameError {
    NoSuchVault,
    Noop,
    Other,
  }

  pub async fn change_vault_name(
    location: &Location,
    database: &Database,
    vault_id: &UuidV4,
    new_vault_name: &VaultName,
  ) -> Result<(), ChangeVaultNameError> {
    todo!()
  }

  pub enum AddDatumError {
    DuplicateId,
    NoSuchVault,
    Other,
  }

  pub async fn add_datum(
    location: &Location,
    database: &Database,
    datum_id: &UuidV4,
    datum_text: &VaultDatum,
  ) -> Result<(), AddDatumError> {
    todo!()
  }

  pub enum DeleteDatumError {
    NoSuchDatum,
    NoSuchVault,
    Other,
  }

  pub async fn delete_datum(
    location: &Location,
    database: &Database,
    datum_id: &UuidV4,
  ) -> Result<(), DeleteDatumError> {
    todo!()
  }
}

pub mod procedures {
  use serde::{Serialize, Deserialize};
  use crate::x::{conditionals, UuidV4, Database};
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  enum VaultProtectorCreator {
    CountdownAfterPlea(conditionals::countdown_after_plea::Creator),
  }

  impl VaultProtectorCreator {
    fn create(self) -> VaultProtector {
      match self {
        Self::CountdownAfterPlea(inner) => {
          VaultProtector::CountdownAfterPlea(inner.create())
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct AddVault {
    vault_id: Option<UuidV4>,
    vault_name: VaultName,
    vault_protector: VaultProtectorCreator,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum AddVaultReturn {
    TooManyVaults,
    DuplicateId,
    InternalError,
    Success,
  }

  impl AddVault {
    pub async fn execute(
      self,
      location: &Location,
      database: &Database,
      cross_group_info: &mut CrossVaultGroupInfo,
    ) -> AddVaultReturn {
      if cross_group_info.may_add_another_vault() {
        return AddVaultReturn::TooManyVaults;
      }

      let vault_id_was_created_by_client = self.vault_id.is_some();
      let vault_id = self.vault_id.unwrap_or_else(UuidV4::generate);
      let vault_protector = self.vault_protector.create();

      if let Err(error) = database::add_vault(
        location, 
        database, 
        &vault_id, 
        &self.vault_name, 
        &vault_protector,
      ).await {
        return match error {
          database::AddVaultError::DuplicateId => {
            if vault_id_was_created_by_client {
              AddVaultReturn::DuplicateId
            } else {
              AddVaultReturn::InternalError
            }
          }
          database::AddVaultError::Other => {
            AddVaultReturn::InternalError
          }
        }
      }

      cross_group_info.try_increment_vault_number();
      AddVaultReturn::Success
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct DeleteVault {
    vault_id: UuidV4,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum DeleteVaultReturn {
    NoSuchVault,
    VaultStillProtected,
    InternalError,
    Succes,
  }

  impl DeleteVault {
    pub async fn execute(
      self,
      location: &Location,
      database: &Database,
      cross_group_info: &mut CrossVaultGroupInfo,
    ) -> DeleteVaultReturn {
      if let Err(error) = database::delete_vault(
        location, 
        database, 
        &self.vault_id,
      ).await {
        return match error {
          database::DeleteVaultError::NoSuchVault => {
            DeleteVaultReturn::NoSuchVault
          }
          database::DeleteVaultError::Other => {
            DeleteVaultReturn::InternalError
          }
        };
      }

      cross_group_info.try_decrement_vault_number();
      DeleteVaultReturn::Succes
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ChangeVaultName {
    vault_id: UuidV4,
    new_vault_name: VaultName,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum ChangeVaultNameReturn {
    NoSuchVault,
    Noop,
    InternalError,
    Success,
  }

  impl ChangeVaultName {
    pub async fn execute(
      self,
      location: &Location,
      database: &Database,
    ) -> ChangeVaultNameReturn {
      let Err(error) = database::change_vault_name(
        location, 
        database, 
        &self.vault_id, 
        &self.new_vault_name,
      ).await else {
        return ChangeVaultNameReturn::Success;
      };

      match error {
        database::ChangeVaultNameError::NoSuchVault => {
          ChangeVaultNameReturn::NoSuchVault
        }
        database::ChangeVaultNameError::Noop => {
          ChangeVaultNameReturn::Noop
        }
        database::ChangeVaultNameError::Other => {
          ChangeVaultNameReturn::InternalError
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct AddDatum {
    vault_id: UuidV4,
    datum: VaultDatum,
    datum_id: Option<UuidV4>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum AddDatumReturn {
    TooManyData,
    NoSuchVault,
    DuplicateId,
    InternalError,
    Success,
  }

  impl AddDatum {
    pub async fn execute(
      self,
      location: &Location,
      database: &Database,
      cross_group_info: &mut CrossVaultGroupInfo,
    ) -> AddDatumReturn {
      if cross_group_info.may_add_another_datum() {
        return AddDatumReturn::TooManyData;
      }

      let datum_id_was_created_by_client = self.datum_id.is_some();
      let datum_id = self.datum_id.unwrap_or_else(UuidV4::generate);

      let Err(error) = database::add_datum(
        location, 
        database, 
        &datum_id, 
        &self.datum
      ).await else {
        cross_group_info.try_increment_data_number();
        return AddDatumReturn::Success;
      };

      match error {
        database::AddDatumError::DuplicateId => {
          if datum_id_was_created_by_client {
            AddDatumReturn::DuplicateId
          } else {
            AddDatumReturn::InternalError
          }
        }
        database::AddDatumError::NoSuchVault => {
          AddDatumReturn::NoSuchVault
        }
        database::AddDatumError::Other => {
          AddDatumReturn::InternalError
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct DeleteDatum {
    vault_id: UuidV4,
    datum_id: UuidV4,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum DeleteDatumReturn {
    NoSuchVault,
    NoSuchDatum,
    InternalError,
    Success,
  }

  impl DeleteDatum {
    pub async fn delete_datum(
      self,
      location: &Location,
      database: &Database,
      cross_group_info: &mut CrossVaultGroupInfo,
    ) -> DeleteDatumReturn {
      let Err(error) = database::delete_datum(
        location, 
        database, 
        &self.datum_id,
      ).await else {
        cross_group_info.try_decrement_data_number();
        return DeleteDatumReturn::Success;
      };

      match error {
        database::DeleteDatumError::NoSuchDatum => {
          DeleteDatumReturn::NoSuchDatum
        }
        database::DeleteDatumError::NoSuchVault => {
          DeleteDatumReturn::NoSuchVault
        }
        database::DeleteDatumError::Other => {
          DeleteDatumReturn::InternalError
        }
      }
    }
  }
}
use serde::{Deserialize, Serialize};
use crate::x::{Database, UuidV4, conditionals};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum VaultProtectorCreator {
  CountdownAfterPlea(conditionals::countdown_after_plea::Creator),
}

impl VaultProtectorCreator {
  fn create(self) -> VaultProtector {
    match self {
      Self::CountdownAfterPlea(inner) => VaultProtector::CountdownAfterPlea(inner.create()),
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
    )
    .await
    {
      return match error {
        database::AddVaultError::DuplicateId => {
          if vault_id_was_created_by_client {
            AddVaultReturn::DuplicateId
          } else {
            AddVaultReturn::InternalError
          }
        }
        database::AddVaultError::Other => AddVaultReturn::InternalError,
      };
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
    if let Err(error) = database::delete_vault(location, database, &self.vault_id).await {
      return match error {
        database::DeleteVaultError::NoSuchVault => DeleteVaultReturn::NoSuchVault,
        database::DeleteVaultError::Other => DeleteVaultReturn::InternalError,
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
  pub async fn execute(self, location: &Location, database: &Database) -> ChangeVaultNameReturn {
    let Err(error) =
      database::change_vault_name(location, database, &self.vault_id, &self.new_vault_name).await
    else {
      return ChangeVaultNameReturn::Success;
    };

    match error {
      database::ChangeVaultNameError::NoSuchVault => ChangeVaultNameReturn::NoSuchVault,
      database::ChangeVaultNameError::Noop => ChangeVaultNameReturn::Noop,
      database::ChangeVaultNameError::Other => ChangeVaultNameReturn::InternalError,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDatum {
  vault_id: UuidV4,
  datum: Datum,
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

    let Err(error) =
      database::add_datum(location, database, &self.vault_id, &datum_id, &self.datum).await
    else {
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
      database::AddDatumError::NoSuchVault => AddDatumReturn::NoSuchVault,
      database::AddDatumError::Other => AddDatumReturn::InternalError,
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
    let Err(error) = database::delete_datum(location, database, &self.datum_id).await else {
      cross_group_info.try_decrement_data_number();
      return DeleteDatumReturn::Success;
    };

    match error {
      database::DeleteDatumError::NoSuchDatum => DeleteDatumReturn::NoSuchDatum,
      database::DeleteDatumError::NoSuchVault => DeleteDatumReturn::NoSuchVault,
      database::DeleteDatumError::Other => DeleteDatumReturn::InternalError,
    }
  }
}

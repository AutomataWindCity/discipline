use serde::{Deserialize, Serialize};
use crate::x::{Daemon, UuidV4, conditionals};
use crate::x::block_info_access::*;

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
  user_id: UuidV4,
  vault_id: Option<UuidV4>,
  vault_name: VaultName,
  vault_protector: VaultProtectorCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddVaultReturn {
  NoSuchUser,
  TooManyVaults,
  DuplicateId,
  InternalError,
  Success,
}

impl AddVault {
  pub async fn execute(self, daemon: &Daemon) -> AddVaultReturn {
    let mut info = daemon.state.vaults_singleton.write().await;
    if !info.may_add_another_vault() {
      return AddVaultReturn::TooManyVaults;
    }

    let vault_id_is_created_by_client = self.vault_id.is_some();
    let vault_id = self.vault_id.unwrap_or_else(UuidV4::generate);
    let vault_protector = self.vault_protector.create();

    if let Err(error) = database::add_vault(
      &daemon.database,
      &self.user_id,
      &vault_id,
      &self.vault_name,
      &vault_protector,
    )
    .await
    {
      return match error {
        database::AddVaultError::DuplicateId => {
          if vault_id_is_created_by_client {
            AddVaultReturn::DuplicateId
          } else {
            AddVaultReturn::InternalError
          }
        }
        database::AddVaultError::Other => AddVaultReturn::InternalError,
      };
    }

    info.try_increment_vault_number();
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
  pub async fn execute(self, daemon: &Daemon) -> DeleteVaultReturn {
    if let Err(error) = database::delete_vault(&daemon.database, &self.vault_id).await {
      return match error {
        database::DeleteVaultError::NoSuchVault => DeleteVaultReturn::NoSuchVault,
        database::DeleteVaultError::Other => DeleteVaultReturn::InternalError,
      };
    }

    let mut info = daemon.state.vaults_singleton.write().await;
    info.try_decrement_vault_number();
    DeleteVaultReturn::Succes
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVaultName {
  vault_id: UuidV4,
  new_vault_name: VaultName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetVaultNameReturn {
  NoSuchVault,
  Noop,
  InternalError,
  Success,
}

impl SetVaultName {
  pub async fn execute(self, daemon: &Daemon) -> SetVaultNameReturn {
    let Err(error) = database::set_vault_name(
      &daemon.database, 
      &self.vault_id, 
      &self.new_vault_name
    ).await else {
      return SetVaultNameReturn::Success;
    };

    match error {
      database::SetVaultNameError::NoSuchVault => SetVaultNameReturn::NoSuchVault,
      database::SetVaultNameError::Noop => SetVaultNameReturn::Noop,
      database::SetVaultNameError::Other => SetVaultNameReturn::InternalError,
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
  pub async fn execute(self, daemon: &Daemon) -> AddDatumReturn {
    let mut info = daemon.state.vaults_singleton.write().await;
    if info.may_add_another_datum() {
      return AddDatumReturn::TooManyData;
    }

    let datum_id_is_created_by_client = self.datum_id.is_some();
    let datum_id = self.datum_id.unwrap_or_else(UuidV4::generate);

    let Err(error) = database::add_datum(
      &daemon.database, 
      &self.vault_id, 
      &datum_id, 
      &self.datum
    ).await else {
      info.try_increment_data_number();
      return AddDatumReturn::Success;
    };

    match error {
      database::AddDatumError::DuplicateId => {
        if datum_id_is_created_by_client {
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
  pub async fn execute(self, daemon: &Daemon) -> DeleteDatumReturn {
    let mut info = daemon.state.vaults_singleton.write().await;

    let Err(error) = database::delete_datum(
      &daemon.database, 
      &self.datum_id,
    ).await else {
      info.try_decrement_data_number();
      return DeleteDatumReturn::Success;
    };

    match error {
      database::DeleteDatumError::NoSuchDatum => DeleteDatumReturn::NoSuchDatum,
      database::DeleteDatumError::NoSuchVault => DeleteDatumReturn::NoSuchVault,
      database::DeleteDatumError::Other => DeleteDatumReturn::InternalError,
    }
  }
}

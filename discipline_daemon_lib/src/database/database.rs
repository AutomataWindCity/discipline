use std::path::PathBuf;
use crate::x::TextualError;
use super::Connection;

pub struct Database {
  pub connection: Connection,
  pub user_collection: crate::x::users::database::Collection,
  pub user_block_info_vault_collection: crate::x::regulation::block_info_access::database::VaultCollection,
  pub user_block_info_datum_collection: crate::x::regulation::block_info_access::database::DatumCollection,
  pub user_device_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_account_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_internet_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
}

impl Database {
  pub fn open(database_directory: PathBuf) -> Result<Self, TextualError> {
    let connection = Connection::open(database_directory)?;

    let user_collection = crate
      ::x 
      ::users 
      ::database 
      ::Collection
      ::new("UserCollection");

    let user_block_info_vault_collection = crate
      ::x
      ::regulation
      ::block_info_access
      ::database
      ::VaultCollection
      ::new("UserBlockInfoVaultCollection");

    let user_block_info_datum_collection = crate
      ::x
      ::regulation
      ::block_info_access
      ::database
      ::DatumCollection
      ::new("UserBlockInfoDatumCollection");

    let user_device_access_regulation_rule_collection = crate
      ::x
      ::rules
      ::database
      ::user_rule_collection
      ::Collection
      ::new("UserDeviceAccessRegulationRuleCollection");

    let user_account_access_regulation_rule_collection = crate
      ::x
      ::rules
      ::database
      ::user_rule_collection
      ::Collection
      ::new("UserAccountAccessRegulationRuleCollection");
    
    let user_internet_access_regulation_rule_collection = crate
      ::x
      ::rules
      ::database
      ::user_rule_collection
      ::Collection
      ::new("UserInternetAccessRegulationRuleCollection");

    Ok(Self {
      connection,
      user_collection,
      user_block_info_vault_collection,
      user_block_info_datum_collection,
      user_device_access_regulation_rule_collection,
      user_account_access_regulation_rule_collection,
      user_internet_access_regulation_rule_collection,
    })
  }
}
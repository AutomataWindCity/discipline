use std::path::PathBuf;
use crate::x::{TextualError, users, rules, state};
use super::{SqlCode, Connection};

pub struct Database {
  pub connection: Connection,
  pub user_collection: crate::x::users::database::Collection,
  pub user_block_info_vault_collection: crate::x::regulation::block_info_access::database::VaultCollection,
  pub user_block_info_datum_collection: crate::x::regulation::block_info_access::database::DatumCollection,
  pub user_device_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_account_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_internet_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub singleton_collection: crate::x::state::database::Collection,
}

impl Database {
  pub async fn open(database_directory: PathBuf) -> Result<Self, TextualError> {
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

    let singleton_collection = crate
      ::x
      ::state
      ::database
      ::Collection
      ::new("Singleton");

    let mut code = SqlCode::new();
    users::database::write_initialize(&mut code, &user_collection);
    rules::database::user_rule_collection::write_initialize(&mut code, &user_device_access_regulation_rule_collection);
    rules::database::user_rule_collection::write_initialize(&mut code, &user_account_access_regulation_rule_collection);
    rules::database::user_rule_collection::write_initialize(&mut code, &user_internet_access_regulation_rule_collection);
    state::database::write_initialize(&mut code, &singleton_collection);
    connection.execute_2(&code).await.map_err(|error| {
      TextualError::new("Opening a SQLite database connection")
        .with_message("A SQLite error occured while ensuring the database is initialized")
        .with_attachement_display("SQLite code", code.as_str())
        .with_attachement_display("SQLite error", error)
    })?;

    Ok(Self {
      connection,
      singleton_collection,
      user_collection,
      user_block_info_vault_collection,
      user_block_info_datum_collection,
      user_device_access_regulation_rule_collection,
      user_account_access_regulation_rule_collection,
      user_internet_access_regulation_rule_collection,
    })
  }
}
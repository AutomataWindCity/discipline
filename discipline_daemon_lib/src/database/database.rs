use std::path::PathBuf;
use crate::x::{TextualError, database};
use super::{SqlCode, Connection};

pub struct Database {
  pub connection: Connection,
  pub user_collection: database::users::Collection,
  pub user_block_info_vault_table: database::block_info_access::VaultTable,
  pub user_block_info_datum_collection: database::block_info_access::DatumTable,
  pub user_device_access_regulation_rule_table: database::rules::user_rule_table::Table,
  pub user_account_access_regulation_rule_table: database::rules::user_rule_table::Table,
  pub user_internet_access_regulation_rule_table: database::rules::user_rule_table::Table,
  pub singleton_table: database::singleton::SingletonTable,
}

impl Database {
  pub async fn open(database_directory: PathBuf) -> Result<Self, TextualError> {
    let connection = Connection::open(database_directory)?;

    let user_collection = database
      ::users
      ::Collection
      ::new("UserCollection");

    let user_block_info_vault_table = database
      ::block_info_access
      ::VaultTable
      ::new("UserBlockInfoVaultCollection");

    let user_block_info_datum_collection = database
      ::block_info_access
      ::DatumTable
      ::new("UserBlockInfoDatumCollection");

    let user_device_access_regulation_rule_table = database
      ::rules
      ::user_rule_table
      ::Table
      ::new("UserDeviceAccessRegulationRuleCollection");

    let user_account_access_regulation_rule_table = database
      ::rules
      ::user_rule_table
      ::Table
      ::new("UserAccountAccessRegulationRuleCollection");
    
    let user_internet_access_regulation_rule_table = database
      ::rules
      ::user_rule_table
      ::Table
      ::new("UserInternetAccessRegulationRuleCollection");

    let singleton_collection = database
      ::singleton
      ::SingletonTable
      ::new("Singleton");

    let mut code = SqlCode::new();
    database::users::write_initialize(&mut code, &user_collection);
    database::rules::user_rule_table::write_initialize(&mut code, &user_device_access_regulation_rule_table);
    database::rules::user_rule_table::write_initialize(&mut code, &user_account_access_regulation_rule_table);
    database::rules::user_rule_table::write_initialize(&mut code, &user_internet_access_regulation_rule_table);
    database::block_info_access::write_initialize_datum_table(&mut code, &user_block_info_datum_collection);
    database::block_info_access::write_initialize_vault_table(&mut code, &user_block_info_vault_table);
    database::singleton::write_initialize(&mut code, &singleton_collection);

    connection.execute_2(&code).await.map_err(|error| {
      TextualError::new("Opening a SQLite database connection")
        .with_message("A SQLite error occured while ensuring the database is initialized")
        .with_attachement_display("SQLite code", code.as_str())
        .with_attachement_display("SQLite error", error)
    })?;

    Ok(Self {
      connection,
      singleton_table: singleton_collection,
      user_collection,
      user_block_info_vault_table,
      user_block_info_datum_collection,
      user_device_access_regulation_rule_table,
      user_account_access_regulation_rule_table,
      user_internet_access_regulation_rule_table,
    })
  }
}
use std::path::PathBuf;
use super::Connection;

pub struct Database {
  pub connection: Connection,
  pub user_block_info_vault_collection: crate::regulation::block_info_access::database::VaultCollection,
  pub user_block_info_datum_collection: crate::regulation::block_info_access::database::DatumCollection,
  pub user_device_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_account_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
  pub user_internet_access_regulation_rule_collection: crate::x::rules::database::user_rule_collection::Collection,
}

impl Database {
  pub fn open(database_directory: PathBuf) {
    let connection = Connection;
    // database_directory.join("data.sqlite")
  }
}
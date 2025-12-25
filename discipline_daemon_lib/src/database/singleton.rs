use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::x::{Database, MonotonicClock, RuleGroup, State, TextualError, User, UserGroup, UuidV4, block_info_access, regulation, rules, users};
use crate::x::database::*;
use crate::x::database;

static ID: Key = Key::new("ID");

static CLOCK_MILLISECONDS: Key = Key::new("ClockMilliseconds");
static CLOCK_PREVIOUS_SYNCHRONIZATION_TIME: Key = Key::new("ClockPreviousSynchronizationTime");
static CLOCK_SYNCHRONIZATION_INTERVAL: Key = Key::new("ClockSynchronizationInterval");

static RULES_RULE_NUMBER: Key = Key::new("RulesNumber");
static RULES_MAXIMUM_RULE_NUMBER: Key = Key::new("RulesMaximumNumber");

static BLOCK_INFO_ACCESS_VAULT_NUMBER: Key = Key::new("BlockInfoAccessVaultNumber");
static BLOCK_INFO_ACCESS_VAULT_MAXIMUM_NUMBER: Key = Key::new("BlockInfoAccessMaximumVaultNumber");
static BLOCK_INFO_ACCESS_DATA_NUMBER: Key = Key::new("BlockInfoAccessDatumNumber");
static BLOCK_INFO_ACCESS_DATA_MAXIMUM_NUMBER: Key = Key::new("BlockInfoAccessMaximumDatumNumber");

static USERS_MAXIMUM_USER_NUMBER: Key = Key::new("MaximumUserNumber");

pub struct SingletonSchema {
  id: Key,
  clock_singleton: MonotonicClockSchema,
  users_singleton: database::users::SingletonSchema,
  rules_singleton: database::rules::CrossRuleGroupInfoSchema,
  vaults_singleton: database::block_info_access::SingletonSchema,
}

impl SingletonSchema {
  pub fn new() -> Self {
    Self {
      id: ID,
      clock_singleton: MonotonicClockSchema::new(
        CLOCK_MILLISECONDS,
        CLOCK_PREVIOUS_SYNCHRONIZATION_TIME,
        CLOCK_SYNCHRONIZATION_INTERVAL,
      ),
      rules_singleton: database::rules::CrossRuleGroupInfoSchema::new(
        RULES_RULE_NUMBER, 
        RULES_MAXIMUM_RULE_NUMBER,
      ),
      vaults_singleton: database::block_info_access::SingletonSchema::new(
        BLOCK_INFO_ACCESS_VAULT_NUMBER, 
        BLOCK_INFO_ACCESS_VAULT_MAXIMUM_NUMBER, 
        BLOCK_INFO_ACCESS_DATA_NUMBER, 
        BLOCK_INFO_ACCESS_DATA_MAXIMUM_NUMBER,
      ),
      users_singleton: database::users::SingletonSchema::new(
        USERS_MAXIMUM_USER_NUMBER,
      ),
    }
  }
}

pub struct SingletonTable {
  name: String,
  schema: SingletonSchema,
}

impl SingletonTable {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: SingletonSchema::new(),
    }
  }
}

pub struct Singleton {
  id: u8,
  clock_singleton: MonotonicClock,
  rules_singleton: rules::RulesSingleton,
  users_singleton: users::UsersSingleton,
  vaults_singleton: block_info_access::VaultsSingleton,
}

impl Default for Singleton {
  fn default() -> Self {
    Self {
      id: 1,
      clock_singleton: Default::default(),
      rules_singleton: Default::default(),
      users_singleton: Default::default(),
      vaults_singleton: Default::default(),
    }
  }
}

impl ReadCompoundValue for Singleton {
  type Schema = SingletonSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Singleton { 
      // TODO: Verify that 'id' is '1'.
      id: source.read_scalar_value(schema.id)?,
      clock_singleton: source.read_compound_value(&schema.clock_singleton)?, 
      users_singleton: source.read_compound_value(&schema.users_singleton)?,
      rules_singleton: source.read_compound_value(&schema.rules_singleton)?, 
      vaults_singleton: source.read_compound_value(&schema.vaults_singleton)?,
    })
  }
}

impl WriteCompoundValue for Singleton {
  type Schema = SingletonSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.id, &value.id);
    destination.write_compound_value(&schema.clock_singleton, &value.clock_singleton);
    destination.write_compound_value(&schema.users_singleton, &value.users_singleton);
    destination.write_compound_value(&schema.rules_singleton, &value.rules_singleton);
    destination.write_compound_value(&schema.vaults_singleton, &value.vaults_singleton);
  }
}

pub fn write_initialize(
  code: &mut SqlCode,
  collection: &SingletonTable,
) {
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&collection.name);
  code.write(" (");

  code.write_key(ID);
  code.write(" INTEGER PRIMARY KEY, ");

  code.write_key(CLOCK_MILLISECONDS);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(CLOCK_PREVIOUS_SYNCHRONIZATION_TIME);
  code.write(" INTEGER, ");
  code.write_key(CLOCK_SYNCHRONIZATION_INTERVAL);
  code.write(" INTEGER NOT NULL, ");

  code.write_key(RULES_RULE_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(RULES_MAXIMUM_RULE_NUMBER);
  code.write(" INTEGER NOT NULL, ");

  code.write_key(BLOCK_INFO_ACCESS_VAULT_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_VAULT_MAXIMUM_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_DATA_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_DATA_MAXIMUM_NUMBER);
  code.write(" INTEGER NOT NULL, ");

  code.write_key(USERS_MAXIMUM_USER_NUMBER);
  code.write(" INTEGER NOT NULL) WITHOUT ROWID;");
}

fn write_initialize_singleton(
  code: &mut SqlCode,
  table: &SingletonTable,
) {
  code.write("INSERT INTO ");
  code.write(&table.name);
  code.write_compound_value_for_insert(
    &table.schema, 
    &Singleton::default(),
  );
  code.write_char(';');
}

fn write_select_singleton(
  code: &mut SqlCode,
  table: &SingletonTable,
) {
  code.write("SELECT * FROM ");
  code.write(&table.name);
  code.write(" WHERE ");
  code.write_key(ID);
  code.write(" = 1;");
}

pub async fn select_singleton(
  database: &Database,
) -> Result<Singleton, TextualError> {
  let mut code = SqlCode::new();
  write_select_singleton(
    &mut code, 
    &database.singleton_table,
  );

  let connection = database.connection.lock().await;

  let singleton = connection
    .get_one_or_none(
      &code, 
      &database.singleton_table.schema,
    )
    .map_err(|error| {
      error.with_context("Selecting the singleton from the database")
    })?;

  if let Some(singleton) = singleton {
    return Ok(singleton);
  }

  let singleton = Singleton::default();
  let mut code = SqlCode::new();
  write_initialize_singleton(
    &mut code, 
    &database.singleton_table,
  );

  connection
    .execute_or_textual_error(&code)
    .map_err(|error| {
      error
        .with_context("Selecting the singleton from the database")
        .with_message("The singleton was not initialized. We initialize it the first time we select it. An error occured while initialization.")
    })?;

  Ok(singleton)
}

pub async fn select_state(database: &Database) -> Result<State, TextualError> {
  let state: Singleton = select_singleton(database).await?;
  
  let mut block_device_access_rule_groups = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  database::rules::user_rule_table::select_all_rules(
    &database.connection, 
    &database.user_device_access_regulation_rule_table, 
    |row| {
      if let Some(rule_group) = block_device_access_rule_groups.get_mut(&row.user_id) {
        rule_group.insert(row.rule_id, row.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(row.rule_id, row.rule);
        block_device_access_rule_groups.insert(row.user_id, rule_group);
      }
    },
  ).await?;

  let mut block_account_access_rule_groups = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  database::rules::user_rule_table::select_all_rules(
    &database.connection, 
    &database.user_account_access_regulation_rule_table, 
    |row| {
      if let Some(rule_group) = block_account_access_rule_groups.get_mut(&row.user_id) {
        rule_group.insert(row.rule_id, row.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(row.rule_id, row.rule);
        block_account_access_rule_groups.insert(row.user_id, rule_group);
      }
    },
  ).await?;

  let mut block_internet_access_rule_group = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  database::rules::user_rule_table::select_all_rules(
    &database.connection, 
    &database.user_internet_access_regulation_rule_table, 
    |row| {
      if let Some(rule_group) = block_internet_access_rule_group.get_mut(&row.user_id) {
        rule_group.insert(row.rule_id, row.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(row.rule_id, row.rule);
        block_internet_access_rule_group.insert(row.user_id, rule_group);
      }
    },
  ).await?;

  let mut users = HashMap::new();
  database::users::get_all_users(database, |item| {
    let block_device_access_rule_group = match block_device_access_rule_groups.remove(&item.id) {
      None => {
        RuleGroup::new()
      }
      Some(rules) => {
        RuleGroup::construct(rules)
      }
    };
    let block_account_access_rule_group = match block_account_access_rule_groups.remove(&item.id) {
      None => {
        RuleGroup::new()
      }
      Some(rules) => {
        RuleGroup::construct(rules)
      }
    };
    let block_internet_access_rule_group = match block_internet_access_rule_group.remove(&item.id) {
      None => {
        RuleGroup::new()
      }
      Some(rules) => {
        RuleGroup::construct(rules)
      }
    };

    let regulation_info = regulation::PerUserInfo::construct(
      regulation::block_device_access::Regulation::construct(block_device_access_rule_group),
      regulation::block_account_access::Regulation::construct(block_account_access_rule_group),
      regulation::block_internet_access::Regulation::construct(block_internet_access_rule_group),
    );

    let user = Arc::new(RwLock::new(User::construct(
      regulation_info,
      item.operating_system_info,
    )));

    users.insert(item.id, user);
  }).await?;

  Ok(State {
    clock: Arc::new(RwLock::const_new(state.clock_singleton)),
    users: Arc::new(RwLock::const_new(UserGroup::construct(users))),
    users_singleton: Arc::new(RwLock::const_new(state.users_singleton)),
    rules_singleton: Arc::new(RwLock::const_new(state.rules_singleton)),
    vaults_singleton: Arc::new(RwLock::const_new(state.vaults_singleton)),
  })
}
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::x::{Database, MonotonicClock, RuleGroup, State, TextualError, User, UserGroup, UuidV4, block_info_access, monotonic, regulation, rules, users};
use crate::x::database::*;

static ID: Key = Key::new("ID");

static CLOCK_MILLISECONDS: Key = Key::new("ClockMilliseconds");

static RULES_RULE_NUMBER: Key = Key::new("RulesNumber");
static RULES_MAXIMUM_RULE_NUMBER: Key = Key::new("MaximumRulesNumber");

static BLOCK_INFO_ACCESS_VAULT_NUMBER: Key = Key::new("BlockInfoAccessVaultNumber");
static BLOCK_INFO_ACCESS_MAXIMUM_VAULT_NUMBER: Key = Key::new("BlockInfoAccessMaximumVaultNumber");

static BLOCK_INFO_ACCESS_DATUM_NUMBER: Key = Key::new("BlockInfoAccessDatumNumber");
static BLOCK_INFO_ACCESS_MAXIMUM_DATUM_NUMBER: Key = Key::new("BlockInfoAccessMaximumDatumNumber");

static USERS_MAXIMUM_USER_NUMBER: Key = Key::new("MaximumUserNumber");

pub struct Schema {
  clock: monotonic::database::Schema,
  rules: rules::database::CrossRuleGroupInfoSchema,
  block_info_access: block_info_access::database::CrossVaultGroupInfoSchema,
  maximum_user_number: Key,
}

impl Schema {
  pub fn new() -> Self {
    Self {
      clock: monotonic::database::Schema::new(
        CLOCK_MILLISECONDS,
      ),
      rules: rules::database::CrossRuleGroupInfoSchema::new(
        RULES_RULE_NUMBER, 
        RULES_MAXIMUM_RULE_NUMBER,
      ),
      block_info_access: block_info_access::database::CrossVaultGroupInfoSchema::new(
        BLOCK_INFO_ACCESS_VAULT_NUMBER, 
        BLOCK_INFO_ACCESS_MAXIMUM_VAULT_NUMBER, 
        BLOCK_INFO_ACCESS_DATUM_NUMBER, 
        BLOCK_INFO_ACCESS_MAXIMUM_DATUM_NUMBER,
      ),
      maximum_user_number: USERS_MAXIMUM_USER_NUMBER,
    }
  }
}

pub struct Collection {
  name: String,
  schema: Schema,
}

impl Collection {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: Schema::new(),
    }
  }
}

pub struct NormalizedState {
  clock: MonotonicClock,
  rules: rules::CrossGroupInfo,
  block_info_access: block_info_access::CrossVaultGroupInfo,
  maximum_user_number: usize,
}

impl ReadCompoundValue for NormalizedState {
  type Schema = Schema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(NormalizedState { 
      clock: source.read_compound_value(&schema.clock)?, 
      rules: source.read_compound_value(&schema.rules)?, 
      block_info_access: source.read_compound_value(&schema.block_info_access)?,
      maximum_user_number: source.read_scalar_value(schema.maximum_user_number)?,
    })
  }
}

pub fn write_initialize(
  code: &mut SqlCode,
  collection: &Collection,
) {
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&collection.name);
  code.write(" (");
  code.write_key(ID);
  code.write(" INTEGER PRIMARY KEY, ");
  code.write_key(CLOCK_MILLISECONDS);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(RULES_RULE_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(RULES_MAXIMUM_RULE_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_VAULT_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_MAXIMUM_VAULT_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_DATUM_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(BLOCK_INFO_ACCESS_MAXIMUM_DATUM_NUMBER);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(USERS_MAXIMUM_USER_NUMBER);
  code.write(" INTEGER NOT NULL);");
}

pub async fn load(database: &Database) -> Result<State, TextualError> {
  let mut code = SqlCode::new();
  code.write("SELECT * FROM ");
  code.write(&database.singleton_collection.name);
  code.write(" WHERE ");
  code.write_key(ID);
  code.write(" = 0 LIMIT 1;");

  let state: NormalizedState = database.connection.get_one(
    &code, 
    &database.singleton_collection.schema,
  ).await?;

  
  let mut block_device_access_rule_groups = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  rules::database::user_rule_collection::get_all_rules(
    &database.connection, 
    &database.user_device_access_regulation_rule_collection, 
    |item| {
      if let Some(rule_group) = block_device_access_rule_groups.get_mut(&item.user_id) {
        rule_group.insert(item.rule_id, item.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(item.rule_id, item.rule);
        block_device_access_rule_groups.insert(item.user_id, rule_group);
      }
    },
  ).await?;

  let mut block_account_access_rule_groups = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  rules::database::user_rule_collection::get_all_rules(
    &database.connection, 
    &database.user_account_access_regulation_rule_collection, 
    |item| {
      if let Some(rule_group) = block_account_access_rule_groups.get_mut(&item.user_id) {
        rule_group.insert(item.rule_id, item.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(item.rule_id, item.rule);
        block_account_access_rule_groups.insert(item.user_id, rule_group);
      }
    },
  ).await?;

  let mut block_internet_access_rule_group = HashMap::<UuidV4, HashMap<UuidV4, rules::Rule>>::new();
  rules::database::user_rule_collection::get_all_rules(
    &database.connection, 
    &database.user_internet_access_regulation_rule_collection, 
    |item| {
      if let Some(rule_group) = block_internet_access_rule_group.get_mut(&item.user_id) {
        rule_group.insert(item.rule_id, item.rule);
      } else {
        let mut rule_group = HashMap::new();
        rule_group.insert(item.rule_id, item.rule);
        block_internet_access_rule_group.insert(item.user_id, rule_group);
      }
    },
  ).await?;

  let mut users = HashMap::new();
  users::database::get_all_users(database, |item| {
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
    clock: Arc::new(RwLock::const_new(state.clock)),
    rules: Arc::new(RwLock::const_new(state.rules)),
    users: Arc::new(RwLock::const_new(UserGroup::construct(users, state.maximum_user_number))),
    block_info_access: Arc::new(RwLock::const_new(state.block_info_access)),
  })
}
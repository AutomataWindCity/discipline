use crate::x::{TextualError, UuidV4};
use crate::x::rules::*;
use crate::x::database::*;

struct CollectionItem {
  rule_id: UuidV4, 
  user_id: UuidV4, 
  rule: Rule,
}

pub struct CollectionItemSchema {
  rule_id: Key,
  user_id: Key,
  rule: RuleSchema,
}

static USER_ID: Key = Key::new("user_id");
static RULE_ID: Key = Key::new("rule_id");
static RULE_ACTIVATOR_ENUM_TYPE: Key = Key::new("RuleActivatorEnumType");
static RULE_ACTIVATOR_ENUM_DATA_1: Key = Key::new("RuleActivatorEnumData1");
static RULE_ACTIVATOR_ENUM_DATA_2: Key = Key::new("RuleActivatorEnumData2");
static RULE_ACTIVATOR_ENUM_DATA_3: Key = Key::new("RuleActivatorEnumData3");
static RULE_ENABLER_ENUM_TYPE: Key = Key::new("RuleEnablerEnumType");
static RULE_ENABLER_ENUM_DATA_1: Key = Key::new("RuleEnablerEnumData1");
static RULE_ENABLER_ENUM_DATA_2: Key = Key::new("RuleEnablerEnumData2");
static RULE_ENABLER_ENUM_DATA_3: Key = Key::new("RuleEnablerEnumData3");


impl CollectionItemSchema {
  pub fn new() -> Self {
    Self {
      rule_id: RULE_ID,
      user_id: USER_ID,
      rule: RuleSchema::new(
        RULE_ACTIVATOR_ENUM_TYPE, 
        RULE_ACTIVATOR_ENUM_DATA_1, 
        RULE_ACTIVATOR_ENUM_DATA_2, 
        RULE_ACTIVATOR_ENUM_DATA_3, 
        RULE_ENABLER_ENUM_TYPE, 
        RULE_ENABLER_ENUM_DATA_1, 
        RULE_ENABLER_ENUM_DATA_2, 
        RULE_ENABLER_ENUM_DATA_3, 
      ),
    }
  }
}

impl WriteCompoundValue for CollectionItem {
  type Schema = CollectionItemSchema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, &value.user_id);
    writer.write_scalar_value(schema.rule_id, &value.rule_id);
    writer.write_compound_value(&schema.rule, &value.rule);
  }
}

impl ReadCompoundValue for CollectionItem {
  type Schema = CollectionItemSchema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CollectionItem {
      rule: reader.read_compound_value(&schema.rule)?,
      rule_id: reader.read_scalar_value(schema.rule_id)?,
      user_id: reader.read_scalar_value(schema.user_id)?,
    })
  }
}

pub struct UserRuleSerializer<'a> {
  pub rule: &'a Rule,
  pub rule_id: &'a UuidV4,
  pub user_id: &'a UuidV4,
}

impl<'a> CompoundValueWriter for UserRuleSerializer<'a> {
  type Schema = CollectionItemSchema;

  fn write(&self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, self.user_id);
    writer.write_scalar_value(schema.rule_id, self.rule_id);
    writer.write_compound_value(&schema.rule, self.rule);
  }
}

pub struct Collection {
  name: Key,
  schema: CollectionItemSchema,
}

pub struct UserRuleUpdates<'a> {
  updates: CompoundValueWriteDestinationForUpdate,
  collection: &'a Collection,
}

impl<'a> UserRuleUpdates<'a> {
  fn new(collection: &'a Collection) -> Self {
    Self {
      updates: CompoundValueWriteDestinationForUpdate::new(),
      collection,
    }
  }

  fn collection(&self) -> &Collection {
    self.collection
  }

  fn schema(&self) -> &CollectionItemSchema {
    &self.collection.schema
  }
}

pub fn write_add_rule(
  code: &mut SqlCode, 
  collection: &Collection,
  rule_activator: &RuleActivator,
  rule_enabler: &RuleEnabler,
  rule_id: &UuidV4,
  user_id: &UuidV4,
) {
  code.write("INSERT INTO ");
  code.write_key(collection.name);
  code.write(" ");
  // TODO
  code.write_compound_value(&collection.schema.rule.activator, rule_activator);
  code.write_compound_value(&collection.schema.rule.enabler, rule_enabler);
  code.write_scalar_value(rule_id);
  code.write_scalar_value(user_id);
  code.write(";");
}

fn write_delete_rule(
  code: &mut SqlCode,
  collection: &Collection,
  rule_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write_key(collection.name);
  code.write(" WHERE ");
  code.write_key(collection.schema.rule_id);
  code.write(" = ");
  code.write_scalar_value(rule_id);
  code.write(";");
}

fn write_find_all_rules(
  code: &mut SqlCode,
  collection: &Collection,
) {
  code.write("SELECT * FROM ");
  code.write_key(collection.name);
  code.write(";");
}

fn write_update_rule(
  code: &mut SqlCode,
  collection: &Collection,
  rule_id: &UuidV4,
  updates: &CompoundValueWriteDestinationForUpdate,
) {
  let Some(updates) = updates.inner() else {
    return;
  };

  code.write("UPDATE ");
  code.write_key(collection.name);
  code.write(" SET ");
  code.write(updates);
  code.write(" WHERE ");
  code.write_key(collection.schema.rule_id);
  code.write(" = ");
  code.write_scalar_value(rule_id);
  code.write(";");
}

pub async fn add_rule(
  connection: &Connection,
  collection: &Collection,
  rule_activator: &RuleActivator,
  rule_enabler: &RuleEnabler,
  rule_id: &UuidV4, 
  user_id: &UuidV4,
) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_add_rule(&mut code, collection, rule_activator, rule_enabler, rule_id, user_id);
  connection.execute(&code).await
}

pub async fn remove_rule(
  connection: &Connection,
  collection: &Collection,
  rule_id: &UuidV4) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_delete_rule(&mut code, collection, rule_id);
  connection.execute(&code).await
}

pub async fn update_rule(
  connection: &Connection,
  collection: &Collection,
  rule_id: &UuidV4, 
  updates: &CompoundValueWriteDestinationForUpdate,
) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_update_rule(&mut code, collection, rule_id, updates);
  connection.execute(&code).await
}

pub struct UserRuleCollectionProcedures<'a> {
  collection: &'a Collection,
  connection: Connection,
}

impl<'a> UserRuleCollectionProcedures<'a> {
  // pub async fn add_rule(&self, rule: &Rule, rule_id: &UuidV4, user_id: &UuidV4) -> Result<(), ExecuteError> {
  //   let mut code = SqlCode::new();
  //   write_add_rule(&mut code, self.collection, rule, rule_id, user_id);
  //   self.connection.execute(&code).await
  // }

  pub async fn remove_rule(&self, rule_id: &UuidV4) -> Result<(), DbExecuteError> {
    let mut code = SqlCode::new();
    write_delete_rule(&mut code, self.collection, rule_id);
    self.connection.execute(&code).await
  }

  pub async fn update_rule(&self, rule_id: &UuidV4, updates: &CompoundValueWriteDestinationForUpdate) -> Result<(), DbExecuteError> {
    let mut code = SqlCode::new();
    write_update_rule(&mut code, self.collection, rule_id, updates);
    self.connection.execute(&code).await
  }

  pub async fn find_all_rules(&self) {
    
  }
}
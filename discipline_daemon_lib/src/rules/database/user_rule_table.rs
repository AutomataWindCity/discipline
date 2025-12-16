// TODO: Do better error handling and logging

use crate::x::{TextualError, UuidV4};
use crate::x::rules::*;
use crate::x::database::*;

static USER_ID: Key = Key::new("UserId");
static RULE_ID: Key = Key::new("RuleId");
static RULE_ACTIVATOR_ENUM_TYPE: Key = Key::new("RuleActivatorEnumType");
static RULE_ACTIVATOR_ENUM_DATA_1: Key = Key::new("RuleActivatorEnumData1");
static RULE_ACTIVATOR_ENUM_DATA_2: Key = Key::new("RuleActivatorEnumData2");
static RULE_ACTIVATOR_ENUM_DATA_3: Key = Key::new("RuleActivatorEnumData3");
static RULE_ENABLER_ENUM_TYPE: Key = Key::new("RuleEnablerEnumType");
static RULE_ENABLER_ENUM_DATA_1: Key = Key::new("RuleEnablerEnumData1");
static RULE_ENABLER_ENUM_DATA_2: Key = Key::new("RuleEnablerEnumData2");
static RULE_ENABLER_ENUM_DATA_3: Key = Key::new("RuleEnablerEnumData3");

pub struct Row {
  pub rule_id: UuidV4, 
  pub user_id: UuidV4, 
  pub rule: Rule,
}

pub struct Schema {
  rule_id: Key,
  user_id: Key,
  rule: RuleSchema,
}

impl Schema {
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

impl WriteCompoundValue for Row {
  type Schema = Schema;

  fn write(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, &value.user_id);
    writer.write_scalar_value(schema.rule_id, &value.rule_id);
    writer.write_compound_value(&schema.rule, &value.rule);
  }
}

impl ReadCompoundValue for Row {
  type Schema = Schema;

  fn deserialize(reader: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(Row {
      rule: reader.read_compound_value(&schema.rule)?,
      rule_id: reader.read_scalar_value(schema.rule_id)?,
      user_id: reader.read_scalar_value(schema.user_id)?,
    })
  }
}

pub struct UserRuleWriter<'a> {
  pub rule: &'a Rule,
  pub rule_id: &'a UuidV4,
  pub user_id: &'a UuidV4,
}

impl<'a> CompoundValueWriter for UserRuleWriter<'a> {
  type Schema = Schema;

  fn write(&self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.user_id, self.user_id);
    writer.write_scalar_value(schema.rule_id, self.rule_id);
    writer.write_compound_value(&schema.rule, self.rule);
  }
}

pub struct Table {
  name: String,
  schema: Schema,
}

impl Table {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      schema: Schema::new(),
    }
  }
}

pub fn write_initialize(
  code: &mut SqlCode, 
  table: &Table,
) {
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&table.name);
  code.write(" (");
  code.write_key(RULE_ID);
  code.write(" TEXT PRIMARY KEY, ");
  code.write_key(USER_ID);
  code.write(" TEXT NOT NULL, ");
  code.write_key(RULE_ACTIVATOR_ENUM_TYPE);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(RULE_ACTIVATOR_ENUM_DATA_1);
  code.write(", ");
  code.write_key(RULE_ACTIVATOR_ENUM_DATA_2);
  code.write(", ");
  code.write_key(RULE_ACTIVATOR_ENUM_DATA_3);
  code.write(", ");
  code.write_key(RULE_ENABLER_ENUM_TYPE);
  code.write(" INTEGER NOT NULL, ");
  code.write_key(RULE_ENABLER_ENUM_DATA_1);
  code.write(", ");
  code.write_key(RULE_ENABLER_ENUM_DATA_2);
  code.write(", ");
  code.write_key(RULE_ENABLER_ENUM_DATA_3);
  code.write(") WITHOUT ROWID;");
}

pub fn write_insert_rule(
  code: &mut SqlCode, 
  table: &Table,
  rule: &Rule,
  rule_id: &UuidV4,
  user_id: &UuidV4,
) {
  code.write("INSERT INTO ");
  code.write(&table.name);
  code.write(" ");
  code.write_compound_value_with_writer_for_insert(
    &table.schema, 
    &UserRuleWriter {
      user_id,
      rule_id,
      rule,
    }
  );
  code.write(";");
}

fn write_delete_rule(
  code: &mut SqlCode,
  table: &Table,
  rule_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&table.name);
  code.write(" WHERE ");
  code.write_column_equal_value(RULE_ID, rule_id);
  code.write(";");
}

fn write_select_all_rules(
  code: &mut SqlCode,
  table: &Table,
) {
  code.write("SELECT * FROM ");
  code.write(&table.name);
  code.write(";");
}

fn write_set_rule_enabler(
  code: &mut SqlCode,
  table: &Table,
  rule_id: &UuidV4,
  schema: &<RuleEnabler as WriteCompoundValueDifferences>::Schema,
  original_enabler: &RuleEnabler,
  modified_enabler: &RuleEnabler,
) {
  let mut destination = CompoundValueWriteDestinationForUpdate::new();
  WriteCompoundValueDifferences::write_differences(
    original_enabler, 
    modified_enabler, 
    schema,
    &mut destination,
  );
  
  let Some(modifications) = destination.inner() else {
    return;
  };

  code.write("UPDATE ");
  code.write(&table.name);
  code.write(" SET ");
  code.write(modifications);
  code.write(" WHERE ");
  code.write_column_equal_value(RULE_ID, rule_id);
  code.write_char(';');
}

// fn write_update_rule(
//   code: &mut SqlCode,
//   collection: &Collection,
//   rule_id: &UuidV4,
//   updates: &CompoundValueWriteDestinationForUpdate,
// ) {
//   let Some(updates) = updates.inner() else {
//     return;
//   };

//   code.write("UPDATE ");
//   code.write(&collection.name);
//   code.write(" SET ");
//   code.write(updates);
//   code.write(" WHERE ");
//   code.write_key(collection.schema.rule_id);
//   code.write(" = ");
//   code.write_scalar_value(rule_id);
//   code.write(";");
// }

pub async fn insert_rule(
  connection: &Connection,
  table: &Table,
  rule: &Rule,
  rule_id: &UuidV4, 
  user_id: &UuidV4,
) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_insert_rule(
    &mut code, 
    table, 
    rule, 
    rule_id, 
    user_id,
  );

  connection.execute(&code).await
}

pub async fn delete_rule(
  connection: &Connection,
  table: &Table,
  rule_id: &UuidV4,
) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_delete_rule(&mut code, table, rule_id);
  connection.execute(&code).await
}

// pub async fn update_rule(
//   connection: &Connection,
//   collection: &Collection,
//   rule_id: &UuidV4, 
//   updates: &CompoundValueWriteDestinationForUpdate,
// ) -> Result<(), DbExecuteError> {
//   let mut code = SqlCode::new();
//   write_update_rule(&mut code, collection, rule_id, updates);
//   connection.execute(&code).await
// }

pub async fn select_all_rules<T>(
  connection: &Connection,
  table: &Table,
  for_each: T
) -> Result<(), TextualError> 
where 
  T: FnMut(Row)
{
  let mut code = SqlCode::new();
  write_select_all_rules(&mut code, table);

  connection.select_multiple(
    &code, 
    &table.schema, 
    for_each,
  ).await
}

pub async fn set_rule_enabler(
  connection: &Connection,
  table: &Table,
  rule_id: &UuidV4,
  original_enabler: &RuleEnabler,
  modified_enabler: &RuleEnabler,
) -> Result<(), DbExecuteError> {
  let mut code = SqlCode::new();
  write_set_rule_enabler(
    &mut code, 
    table, 
    rule_id, 
    &table.schema.rule.enabler, 
    original_enabler, 
    modified_enabler,
  );

  connection
    .lock()
    .await
    .execute(&code)
}
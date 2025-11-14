use crate::x::{TextualError, UuidV4};
use crate::x::rules::rules_x::*;
use crate::x::database::*;

struct UserRule {
  rule_id: UuidV4, 
  user_id: UuidV4, 
  rule: RuleX,
}

pub struct UserRuleSchema {
  rule_id: Key,
  user_id: Key,
  rule: RuleSchema,
}

static USER_ID: Key = Key::new("UserId");
static RULE_ID: Key = Key::new("RuleId");
static RULE_IS_ACTIVATED: Key = Key::new("RuleIsActivated");
static RULE_ACTION_CONDITIONAL_ENUM_TYPE: Key = Key::new("RuleActionConditionalEnumType");
static RULE_ACTION_CONDITIONAL_ENUM_DATA_1: Key = Key::new("RuleActionConditionalEnumData1");
static RULE_ACTION_CONDITIONAL_ENUM_DATA_2: Key = Key::new("RuleActionConditionalEnumData2");
static RULE_ACTION_CONDITIONAL_ENUM_DATA_3: Key = Key::new("RuleActionConditionalEnumData3");
static RULE_PROTECTION_CONDITIONAL_ENUM_TYPE: Key = Key::new("RuleProtectionConditionalEnumType");
static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_1: Key = Key::new("RuleProtectionConditionalEnumData1");
static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_2: Key = Key::new("RuleProtectionConditionalEnumData2");
static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_3: Key = Key::new("RuleProtectionConditionalEnumData3");
static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_4: Key = Key::new("RuleProtectionConditionalEnumData4");

impl UserRuleSchema {
  pub fn new() -> Self {
    Self {
      rule_id: RULE_ID,
      user_id: USER_ID,
      rule: RuleSchema::new(
        RULE_ACTION_CONDITIONAL_ENUM_TYPE, 
        RULE_ACTION_CONDITIONAL_ENUM_DATA_1, 
        RULE_ACTION_CONDITIONAL_ENUM_DATA_2, 
        RULE_ACTION_CONDITIONAL_ENUM_DATA_3, 
        RULE_PROTECTION_CONDITIONAL_ENUM_TYPE, 
        RULE_PROTECTION_CONDITIONAL_ENUM_DATA_1, 
        RULE_PROTECTION_CONDITIONAL_ENUM_DATA_2, 
        RULE_PROTECTION_CONDITIONAL_ENUM_DATA_3, 
        RULE_PROTECTION_CONDITIONAL_ENUM_DATA_4, 
        RULE_IS_ACTIVATED,
      ),
    }
  }
}

impl SerializableCompoundValue for UserRule {
  type Schema = UserRuleSchema;

  fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    // writer.write_scalar_value(schema.user_id, &value.user_id);
    // writer.write_scalar_value(schema.rule_id, &value.rule_id);
    // writer.write_compound_value(&schema.rule, &value.rule);
  }
}

impl DeserializableCompoundValue for UserRule {
  type Schema = UserRuleSchema;

  fn deserialize(reader: &mut impl CompoundValueReader, schema: &Self::Schema) -> Result<Self, TextualError> {
    // Ok(UserRule {
    //   rule: reader.read_compound_value(&schema.rule)?,
    //   rule_id: reader.read_scalar_value(schema.rule_id)?,
    //   user_id: reader.read_scalar_value(schema.user_id)?,
    // })
    todo!()
  }
}

pub struct UserRuleSerializer<'a> {
  pub rule: &'a RuleX,
  pub rule_id: &'a UuidV4,
  pub user_id: &'a UuidV4,
}

impl<'a> CompoundValueSerializer for UserRuleSerializer<'a> {
  type Schema = UserRuleSchema;

  fn serialize(&self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
    // writer.write_scalar_value(schema.user_id, self.user_id);
    // writer.write_scalar_value(schema.rule_id, self.rule_id);
    // writer.write_compound_value(&schema.rule, self.rule);
  }
}

pub struct UserRuleCollection {
  name: String,
  schema: UserRuleSchema,
}

pub struct UserRuleUpdates<'a> {
  updates: CollectionItemUpdates,
  collection: &'a UserRuleCollection,
}

impl<'a> UserRuleUpdates<'a> {
  fn new(collection: &'a UserRuleCollection) -> Self {
    Self {
      updates: CollectionItemUpdates::new(),
      collection,
    }
  }

  fn collection(&self) -> &UserRuleCollection {
    self.collection
  }

  fn schema(&self) -> &UserRuleSchema {
    &self.collection.schema
  }
}

fn write_add_rule(
  code: &mut SqlCode, 
  collection: &UserRuleCollection,
  rule: &RuleX,
  rule_id: &UuidV4,
  user_id: &UuidV4,
) {
  code.write("INSERT INTO ");
  code.write(&collection.name);
  code.write(" ");
  serialize_compound_value_with_serializer(
    code, 
    &collection.schema, 
    &UserRuleSerializer { rule, rule_id, user_id }
  );
  code.write(";");
}

fn write_delete_rule(
  code: &mut SqlCode,
  collection: &UserRuleCollection,
  rule_id: &UuidV4,
) {
  code.write("DELETE FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write(collection.schema.rule_id.as_str());
  code.write(" = ");
  serialize_scalar_value(code, rule_id);
  code.write(";");
}

fn write_find_all_rules(
  code: &mut SqlCode,
  collection: &UserRuleCollection,
) {
  code.write("SELECT * FROM ");
  code.write(&collection.name);
  code.write(";");
}

fn write_update_rule(
  code: &mut SqlCode,
  collection: &UserRuleCollection,
  rule_id: &UuidV4,
  updates: &CollectionItemUpdates,
) {
  let Some(updates) = updates.inner() else {
    return;
  };

  code.write("UPDATE ");
  code.write(&collection.name);
  code.write(" SET ");
  code.write(updates);
  code.write(" WHERE ");
  code.write(collection.schema.rule_id.as_str());
  code.write(" = ");
  serialize_scalar_value(code, rule_id);
  code.write(";");
}

pub struct UserRuleCollectionProcedures<'a> {
  collection: &'a UserRuleCollection,
  connection: Connection,
}

impl<'a> UserRuleCollectionProcedures<'a> {
  pub async fn add_rule(&self, rule: &RuleX, rule_id: &UuidV4, user_id: &UuidV4) -> Result<(), ExecuteError> {
    let mut code = SqlCode::new();
    write_add_rule(&mut code, self.collection, rule, rule_id, user_id);
    self.connection.execute(&code).await
  }

  pub async fn remove_rule(&self, rule_id: &UuidV4) -> Result<(), ExecuteError> {
    let mut code = SqlCode::new();
    write_delete_rule(&mut code, self.collection, rule_id);
    self.connection.execute(&code).await
  }

  pub async fn update_rule(&self, rule_id: &UuidV4, updates: &CollectionItemUpdates) -> Result<(), ExecuteError> {
    let mut code = SqlCode::new();
    write_update_rule(&mut code, self.collection, rule_id, updates);
    self.connection.execute(&code).await
  }

  pub async fn find_all_rules(&self) {
    
  }
}


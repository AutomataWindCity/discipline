use serde::{Deserialize, Serialize};
use crate::{rules::rules_x, x::RuleGroup};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeviceAccess {
  pub rules: rules_x::RuleGroupX,
}

impl BlockDeviceAccess {
  pub fn new() -> Self {
    Self {
      rules: rules_x::RuleGroupX::new(100),
    }
  }
}

// pub mod database {
//   use crate::x::database::*;

//   pub struct Schema {
//     rule: RuleSchema,
//     rule_id: Key,
//     user_id: Key,
//   }

//   pub struct Collection {
//     name: String,
//     schema: Schema,
//   }

//   static RULE_ID: Key = Key::new("rule_id");
//   static USER_ID: Key = Key::new("user_id");
//   static RULE_ACTION_CONDITIONAL_ENUM_TYPE: Key = Key::new("rule_action_conditional_enum_type");
//   static RULE_ACTION_CONDITIONAL_ENUM_DATA_1: Key = Key::new("rule_action_conditional_enum_data_1");
//   static RULE_ACTION_CONDITIONAL_ENUM_DATA_2: Key = Key::new("rule_action_conditional_enum_data_2");
//   static RULE_ACTION_CONDITIONAL_ENUM_DATA_3: Key = Key::new("rule_action_conditional_enum_data_3");
//   static RULE_PROTECTION_CONDITIONAL_ENUM_TYPE: Key = Key::new("rule_protection_conditional_enum_type");
//   static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_1: Key = Key::new("rule_protection_conditional_enum_data_1");
//   static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_2: Key = Key::new("rule_protection_conditional_enum_data_2");
//   static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_3: Key = Key::new("rule_protection_conditional_enum_data_3");
//   static RULE_PROTECTION_CONDITIONAL_ENUM_DATA_4: Key = Key::new("rule_protection_conditional_enum_data_4");
//   static RULE_IS_ACTIVATED: Key = Key::new("rule_is_activated");

//   impl Collection {
//     pub fn new(name: impl Into<String>) -> Self {
//       let rule = RuleSchema::new(
//         RULE_ACTION_CONDITIONAL_ENUM_TYPE, 
//         RULE_ACTION_CONDITIONAL_ENUM_DATA_1, 
//         RULE_ACTION_CONDITIONAL_ENUM_DATA_2, 
//         RULE_ACTION_CONDITIONAL_ENUM_DATA_3, 
//         RULE_PROTECTION_CONDITIONAL_ENUM_TYPE, 
//         RULE_PROTECTION_CONDITIONAL_ENUM_DATA_1, 
//         RULE_PROTECTION_CONDITIONAL_ENUM_DATA_2, 
//         RULE_PROTECTION_CONDITIONAL_ENUM_DATA_3, 
//         RULE_PROTECTION_CONDITIONAL_ENUM_DATA_4, 
//         RULE_IS_ACTIVATED,
//       );

//       let schema = Schema {
//         rule,
//         rule_id: RULE_ID,
//         user_id: USER_ID,
//       };

//       Self {
//         name: name.into(),
//         schema,
//       }
//     }
//   }

//   pub fn write_initialization(
//     code: &mut SqlCode,
//     collection: &Collection,
//   ) {
//     code.write("CREATE TABLE IF NOT EXISTS ");
//     code.write(&collection.name);
//     code.write(" (");
//     code.write(USER_ID.as_str());
//     code.write(" TEXT NOT NULL, ");
//     code.write(RULE_ID.as_str());
//     code.write(" TEXT NOT NULL, ");
//     code.write(RULE_ACTION_CONDITIONAL_ENUM_TYPE.as_str());
//     code.write(" INTEGER NOT NULL, ");
//     code.write(RULE_ACTION_CONDITIONAL_ENUM_DATA_1.as_str());
//     code.write(" INTEGER NOT NULL, ");
//   }
// }
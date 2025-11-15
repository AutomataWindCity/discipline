use serde::{Deserialize, Serialize};
use crate::x::RuleGroup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeviceAccess {
  pub rules: RuleGroup,
}

impl BlockDeviceAccess {
  pub fn new() -> Self {
    Self {
      rules: RuleGroup::new(100),
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
//   static RULE_ACTIVATOR_ENUM_TYPE: Key = Key::new("rule_activator_enum_type");
//   static RULE_ACTIVATOR_ENUM_DATA_1: Key = Key::new("rule_activator_enum_data_1");
//   static RULE_ACTIVATOR_ENUM_DATA_2: Key = Key::new("rule_activator_enum_data_2");
//   static RULE_ACTIVATOR_ENUM_DATA_3: Key = Key::new("rule_activator_enum_data_3");
//   static RULE_ENABLER_ENUM_TYPE: Key = Key::new("rule_enabler_enum_type");
//   static RULE_ENABLER_ENUM_DATA_1: Key = Key::new("rule_enabler_enum_data_1");
//   static RULE_ENABLER_ENUM_DATA_2: Key = Key::new("rule_enabler_enum_data_2");
//   static RULE_ENABLER_ENUM_DATA_3: Key = Key::new("rule_enabler_enum_data_3");
//   static RULE_ENABLER_ENUM_DATA_4: Key = Key::new("rule_enabler_enum_data_4");
//   static RULE_IS_ACTIVATED: Key = Key::new("rule_is_activated");

//   impl Collection {
//     pub fn new(name: impl Into<String>) -> Self {
//       let rule = RuleSchema::new(
//         RULE_ACTIVATOR_ENUM_TYPE, 
//         RULE_ACTIVATOR_ENUM_DATA_1, 
//         RULE_ACTIVATOR_ENUM_DATA_2, 
//         RULE_ACTIVATOR_ENUM_DATA_3, 
//         RULE_ENABLER_ENUM_TYPE, 
//         RULE_ENABLER_ENUM_DATA_1, 
//         RULE_ENABLER_ENUM_DATA_2, 
//         RULE_ENABLER_ENUM_DATA_3, 
//         RULE_ENABLER_ENUM_DATA_4, 
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
//     code.write(RULE_ACTIVATOR_ENUM_TYPE.as_str());
//     code.write(" INTEGER NOT NULL, ");
//     code.write(RULE_ACTIVATOR_ENUM_DATA_1.as_str());
//     code.write(" INTEGER NOT NULL, ");
//   }
// }
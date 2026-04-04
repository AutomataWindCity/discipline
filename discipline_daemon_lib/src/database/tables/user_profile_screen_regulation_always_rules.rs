use crate::x::{AlwaysRule, RuleProtector, RuleProtectorType, UuidV4};
use crate::x::database::*;

const TABLE: &'static str = "UserProfileScreenRegulationAlwaysRules";

const ID: &'static str = "id";
const ENABLED: &'static str = "enabled";
const USER_ID: &'static str = "user_id";
const PROTECTOR_TYPE: &'static str = "protector_type";
const PROTECTOR_DURATION: &'static str = "protector_duration";

const PROTECTOR_COUNTDOWN_FROM: &'static str = "protector_countdown_from";
const PROTECTOR_COUNTDOWN_DURATION: &'static str = "protector_countdown_duration";

// pub struct ColumnNames {
//   id: ColumnName,
//   user_id: ColumnName,
//   countdown_from: ColumnName,
//   countdown_duration: ColumnName,
// }

// pub struct ColumnIndexes {
//   id: ColumnIndex,
//   user_id: ColumnIndex,
//   countdown_from: ColumnIndex,
//   countdown_duration: ColumnIndex,
// }

pub struct Table {
  // name: TableName,
  // column_names: ColumnNames,
  // column_indexes: ColumnIndexes,
}

impl Table {
  pub fn write_create_table(
    &self, 
    code: &mut SqlCode,
  ) {
    code.write2("CREATE TABLE IF NOT EXISTS ");
    code.write2(TABLE);
    code.write2("( ");
    code.write2(ID);
    code.write2(" TEXT PRIMARY KEY, ");
    code.write2(USER_ID);
    code.write2(" TEXT NOT NULL, ");
    code.write2(PROTECTOR_TYPE);
    code.write2(" INTEGER NOT NULL, ");
    code.write2(PROTECTOR_DURATION);
    code.write2(" INTEGER NOT NULL, ");
    code.write2(PROTECTOR_COUNTDOWN_FROM);
    code.write2(" INTEGER, ");
    code.write2(PROTECTOR_COUNTDOWN_DURATION);
    code.write2(" INTEGER, ");
    code.write2(") STRICT, WITHOUT ROWID;");
  }

  pub fn write_insert(
    &self,
    code: &mut SqlCode,
    id: &UuidV4,
    user_id: &UuidV4,
    rule: &AlwaysRule,
  ) {
    code.write2("INSERT INTO ");
    code.write2(TABLE);
    code.write2(" VALUES (");
    code.write2(id);
    code.write2(", ");
    code.write2(user_id);
    code.write2(", ");
    code.write2(rule.enabled);
    code.write2(", ");
    match &rule.protector {
      RuleProtector::Countdown(protector) => {
        code.write2(RuleProtectorType::Countdown);
        code.write2(", ");
        code.write2(protector.duration);
        code.write2(", ");
        match &protector.countdown {
          Some(countdown) => {
            code.write2(countdown.from);
            code.write2(", ");
            code.write2(countdown.duration);
          }
          None => {
            code.write2(", ");
            code.write2(SqlNull);
            code.write2(", ");
            code.write2(SqlNull);
          }
        }
      }
      RuleProtector::CountdownAfterPlea(protector) => {
        code.write2(RuleProtectorType::CountdownAfterPlea);
        code.write2(", ");
        code.write2(protector.duration);
        code.write2(", ");
        match &protector.countdown {
          Some(countdown) => {
            code.write2(countdown.from);
            code.write2(", ");
            code.write2(countdown.duration);
          }
          None => {
            code.write2(", ");
            code.write2(SqlNull);
            code.write2(", ");
            code.write2(SqlNull);
          }
        }
      }
    }
  }
}

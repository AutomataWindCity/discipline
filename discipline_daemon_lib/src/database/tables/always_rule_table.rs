#[macro_export]
macro_rules! sql {
  ($dst:expr $(,)?) => {};

    ($dst:expr, { $value:expr }) => {
    $dst.write_value($value);
  };

  ($dst:expr, [ $value:expr ]) => {
    $dst.write_value_ref($value);
  };

  ($dst:expr, $value:literal) => {
    $dst.write_literal($value);
  };

  ($dst:expr, $a:tt $($rest:tt)*) => {{
    $crate::sql!($dst, $a);
    $crate::sql!($dst, $($rest)*);
  }};
}


use crate::IsTextualError;
use crate::x::{AlwaysRule, CountdownAfterPleaConditionalDeactivatingState, CountdownConditionalActivateState, RuleEnabler, RuleEnablerType, UuidV4};
use crate::x::procedures::always_rule::{AlwaysRuleContext, AlwaysRuleLocation};
use crate::x::database::*;
use crate::sql;

const TABLE: &'static str = "AlwaysRules";

const ID: &'static str = "id";
const LOCATION_ID: &'static str = "location_id";
const ENABLER_TYPE: &'static str = "enabler_type";
const ENABLER_DURATION: &'static str = "enabler_duration";
const ENABLER_COUNTDOWN_FROM: &'static str = "enabler_countdown_from";
const ENABLER_COUNTDOWN_DURATION: &'static str = "enabler_countdown_duration";

pub fn write_create_table(code: &mut SqlCode) {
  // code.write2("CREATE TABLE IF NOT EXISTS ");
  // code.write2(TABLE);
  // code.write2("( ");
  // code.write2(ID);
  // code.write2(" TEXT PRIMARY KEY, ");
  // code.write2(LOCATION_ID);
  // code.write2(" TEXT NOT NULL, ");
  // code.write2(ENABLER_TYPE);
  // code.write2(" INTEGER NOT NULL, ");
  // code.write2(ENABLER_DURATION);
  // code.write2(" INTEGER NOT NULL, ");
  // code.write2(ENABLER_COUNTDOWN_FROM);
  // code.write2(" INTEGER, ");
  // code.write2(ENABLER_COUNTDOWN_DURATION);
  // code.write2(" INTEGER, ");
  // code.write2(") STRICT, WITHOUT ROWID;");

  sql!(
    code,

    "CREATE TABLE IF NOT EXISTS " {TABLE} "( "
      {ID} " TEXT PRIMARY KEY, "
      {LOCATION_ID} " TEXT NOT NULL, "
      {ENABLER_TYPE} " INTEGER NOT NULL, "
      {ENABLER_DURATION} " INTEGER NOT NULL, "
      {ENABLER_COUNTDOWN_FROM} " INTEGER, "
      {ENABLER_COUNTDOWN_DURATION} " INTEGER, "
    ") STRICT, WITHOUT ROWID;"
  )
}

pub fn write_insert(
  code: &mut SqlCode,
  location_id: &LocationId,
  rule_id: &UuidV4,
  rule: &AlwaysRule,
) {
  // sql!(
  //   code,
  //   "INSERT INTO " {TABLE} " VALUES ("
  //     [rule_id] ", "
  //     [location_id] ", "
  // )
  // code.write2("INSERT INTO ");
  // code.write2(TABLE);
  // code.write2(" VALUES (");
  // code.write2(rule_id);
  // code.write2(", ");
  // code.write2(location_id);
  // code.write2(", ");
  // match &rule.Enabler {
  //   RuleEnabler::Countdown(enabler) => {
  //     code.write2(RuleEnablerType::Countdown);
  //     code.write2(", ");
  //     code.write2(enabler.duration);
  //     code.write2(", ");
  //     match &enabler.countdown {
  //       Some(countdown) => {
  //         code.write2(countdown.from);
  //         code.write2(", ");
  //         code.write2(countdown.duration);
  //       }
  //       None => {
  //         code.write2(", ");
  //         code.write2(SqlNull);
  //         code.write2(", ");
  //         code.write2(SqlNull);
  //       }
  //     }
  //   }
  //   RuleEnabler::CountdownAfterPlea(enabler) => {
  //     code.write2(RuleEnablerType::CountdownAfterPlea);
  //     code.write2(", ");
  //     code.write2(enabler.duration);
  //     code.write2(", ");
  //     match &enabler.countdown {
  //       Some(countdown) => {
  //         code.write2(countdown.from);
  //         code.write2(", ");
  //         code.write2(countdown.duration);
  //       }
  //       None => {
  //         code.write2(", ");
  //         code.write2(SqlNull);
  //         code.write2(", ");
  //         code.write2(SqlNull);
  //       }
  //     }
  //   }
  // }
}

pub fn insert_rule(
  database: &Database,
  rule_locator: &AlwaysRuleLocation,
  rule_id: &UuidV4,
  rule: &AlwaysRule,
  textual_error: &mut impl IsTextualError,
) -> Result<(), InsertError> {
  let mut code = SqlCode::new();
  // write_insert(&mut code, location_id, rule_id, rule);
  database.connection.execute(&code, textual_error).map_err(|error| match error {
    DbExecuteError::ForiegnKeyViolation => {
      InsertError::Other
    }
    DbExecuteError::PrimaryKeyViolation => {
      InsertError::DuplicateRuleId
    }
    DbExecuteError::Other => {
      InsertError::Other
    }
  })
}

pub fn write_delete(
  code: &mut SqlCode,
  rule_id: &UuidV4,
) {
  // code.write2("DELETE FROM ");
  // code.write2(TABLE);
  // code.write2(" WHERE ");
  // code.write2(ID);
  // code.write2(" = ");
  // code.write2(rule_id);
  // code.write2(";");
}

pub fn delete_rule(
  database: &Database,
  rule_context: &AlwaysRuleContext<'_>,
  rule_id: &UuidV4,
  textual_error: &mut impl IsTextualError,
) -> Result<(), DeleteRule> {
  let mut code = SqlCode::new();
  write_delete(&mut code, rule_id);
  database.connection.execute(&code, textual_error).map_err(|error| match error {
    DbExecuteError::PrimaryKeyViolation => {
      DeleteRule::Other
    }
    DbExecuteError::ForiegnKeyViolation => {
      DeleteRule::Other
    }
    DbExecuteError::Other => {
      DeleteRule::Other
    }
  })
}

pub fn enabler_countdown_after_plea_write_activate(
  code: &mut SqlCode,
  rule_id: &UuidV4,
) {
  // code.write2("UPDATE ");
  // code.write2(TABLE);
  // code.write2(" SET ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_FROM, SqlNull);
  // code.write2(", ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_DURATION, SqlNull);
  // code.write2(" WHERE ");
  // code.write_column_equal_value(ID, rule_id);
  // code.write2(";");
}

pub fn enaber_countdown_after_plea_activate(
  database: &Database,
  rule_id: &UuidV4,
  textual_error: &mut impl IsTextualError,
) -> Result<(), EnablerCountdownAfterPleaActivate> {
  let mut code = SqlCode::new();
  enabler_countdown_after_plea_write_activate(&mut code, rule_id);
  database.connection.execute(&code, textual_error).map_err(|_| {
    EnablerCountdownAfterPleaActivate::Other
  })
}

pub fn enabler_countdown_after_plea_write_deactivate(
  code: &mut SqlCode,
  rule_id: &UuidV4,
  deactivating_state: &CountdownAfterPleaConditionalDeactivatingState,
) {
  // code.write2("UPDATE ");
  // code.write2(TABLE);
  // code.write2(" SET ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_FROM, deactivating_state.countdown.from);
  // code.write2(", ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_DURATION, deactivating_state.countdown.duration);
  // code.write2(";");
}

pub fn enabler_countdown_after_plea_deactivate(
  database: &Database,
  rule_id: &UuidV4,
  deactivating_state: &CountdownAfterPleaConditionalDeactivatingState,
  textual_error: &mut impl IsTextualError,
) -> Result<(), EnablerCountdownAfterPleaDeactivate> {
  let mut code = SqlCode::new();
  enabler_countdown_after_plea_write_deactivate(&mut code, rule_id, deactivating_state);
  database.connection.execute(&code, textual_error).map_err(|_| {
    EnablerCountdownAfterPleaDeactivate::Other
  })
}

pub fn enabler_countdown_write_activate(
  code: &mut SqlCode,
  rule_id: &UuidV4,
  activate_state: &CountdownConditionalActivateState,
) {
  // code.write2("UPDATE ");
  // code.write2(TABLE);
  // code.write2(" SET ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_FROM, activate_state.countdown.from);
  // code.write2(", ");
  // code.write_column_equal_value(ENABLER_COUNTDOWN_DURATION, activate_state.countdown.duration);
  // code.write2(";");
}

pub fn enabler_countdown_activate(
  database: &Database,
  rule_context: &AlwaysRuleContext<'_>,
  rule_id: &UuidV4,
  activate_state: &CountdownConditionalActivateState,
  textual_error: &mut impl IsTextualError,
) -> Result<(), EnablerCountdownActivate> {
  let mut code = SqlCode::new();
  enabler_countdown_write_activate(&mut code, rule_id, activate_state);
  database.connection.execute(&code, textual_error).map_err(|_| {
    EnablerCountdownActivate::Other
  })
}

pub enum InsertError {
  DuplicateRuleId,
  Other,
}

pub enum DeleteRule {
  NoSuchRule,
  Other,
}

pub enum EnablerCountdownAfterPleaActivate {
  NoSuchRule,
  Other,
}

pub enum EnablerCountdownAfterPleaDeactivate {
  NoSuchRule,
  Other,
}

pub enum EnablerCountdownActivate {
  NoSuchRule,
  Other,
}
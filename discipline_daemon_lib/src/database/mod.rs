mod other;

mod utilities;
pub use utilities::*;

mod v2;
// mod other;

// mod tables;

// mod scalars;
// use scalars::countdown::CountdownColumnNames;
// use scalars::time_range::TimeRangeSchema;
// use scalars::monotonic_clock::MonotonicClockColumnNames;
// use scalars::option_type::OptionType;

// mod conditionals;
// pub use conditionals::always_conditional::AlwaysConditionalSchema;
// pub use conditionals::countdown_after_plea_conditional::CountdownAfterPleaConditionalSchema;
// pub use conditionals::countdown_conditional::CountdownConditionalSchema;
// pub use conditionals::time_conditional::TimeConditionalSchema;

// mod launcher;
// // pub mod rules;
// // pub use rules::rule_activator::RuleActivatorSchema;
// // pub use rules::rule_enabler::RuleEnablerSchema;
// // pub use rules::rule::RuleSchema;

// // pub mod collections;
// // pub use collections::user_rule_collection;

// // pub mod rules_x;

// pub trait Transaction {
//   fn code(&mut self) -> &mut SqlCode;
// }

// pub trait WriteCompoundValueDifferences<Other = Self> {
//   type Schema;

//   fn write_differences(
//     original: &Self, 
//     modified: &Other,
//     schema: &Self::Schema,
//     destination: &mut impl CompoundValueWriteDestination,
//   );
// }

// mod database;
// pub use database::Database;
// pub mod singleton;
// pub mod block_info_access;
// pub mod users;
// pub mod rules;

pub mod database;
pub use database::Database;

pub mod tables;
pub use tables::*;

use crate::IsTextualError;
use crate::x::{CountdownConditionalActivateState};
use crate::x::procedures::CountdownConditionalLocation;

use crate::x::{CountdownAfterPleaConditionalDeactivatingState, };
use crate::x::procedures::CountdownAfterPleaConditionalLocation;

pub enum CountdownConditionalDbAdapterError {}

pub struct CountdownConditionalDbAdapter {

}

impl CountdownConditionalDbAdapter {
  pub fn activate(
    &self,
    database: &Database,
    location: &CountdownConditionalLocation,
    activate_state: &CountdownConditionalActivateState,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), CountdownConditionalDbAdapterError> {
    todo!()
  }
}

pub enum CountdownAfterPleaConditionalDbAdapterError {}

pub struct CountdownAfterPleaConditionalDbAdapter {}

impl CountdownAfterPleaConditionalDbAdapter {
  pub fn activate(
    &self,
    database: &Database,
    location: &CountdownAfterPleaConditionalLocation,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), CountdownAfterPleaConditionalDbAdapterError> {
    todo!()
  }
  
  pub fn redactivate(
    &self,
    database: &Database,
    location: &CountdownAfterPleaConditionalLocation,
    re_deactivate_state: &CountdownAfterPleaConditionalDeactivatingState,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), CountdownAfterPleaConditionalDbAdapterError> {
    todo!()
  }
}

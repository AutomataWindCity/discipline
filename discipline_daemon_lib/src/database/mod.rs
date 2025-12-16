mod utilities;
pub use utilities::*;

mod other;

mod chronic;
use chronic::countdown::CountdownSchema;
use chronic::time_range::TimeRangeSchema;

mod conditionals;
pub use conditionals::always_conditional::AlwaysConditionalSchema;
pub use conditionals::countdown_after_plea_conditional::CountdownAfterPleaConditionalSchema;
pub use conditionals::countdown_conditional::CountdownConditionalSchema;
pub use conditionals::time_conditional::TimeConditionalSchema;

// pub mod rules;
// pub use rules::rule_activator::RuleActivatorSchema;
// pub use rules::rule_enabler::RuleEnablerSchema;
// pub use rules::rule::RuleSchema;

// pub mod collections;
// pub use collections::user_rule_collection;

// pub mod rules_x;

pub trait Transaction {
  fn code(&mut self) -> &mut SqlCode;
}

pub trait WriteCompoundValueDifferences<Other = Self> {
  type Schema;

  fn write_differences(
    original: &Self, 
    modified: &Other,
    schema: &Self::Schema,
    destination: &mut impl CompoundValueWriteDestination,
  );
}

mod database;
pub use database::Database;
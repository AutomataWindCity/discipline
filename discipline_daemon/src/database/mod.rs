mod utilities;
pub use utilities::*;

mod other;

mod chronic;
use chronic::countdown::CountdownSchema;
use chronic::time_range::TimeRangeSchema;

mod conditionals;
use conditionals::always_conditional::AlwaysConditionalSchema;
use conditionals::countdown_after_plea_conditional::CountdownAfterPleaConditionalSchema;
use conditionals::countdown_conditional::CountdownConditionalSchema;
use conditionals::time_conditional::TimeConditionalSchema;

pub mod rules;
pub use rules::rule_activator::RuleActivatorSchema;
pub use rules::rule_enabler::RuleEnablerSchema;
pub use rules::rule::RuleSchema;

pub mod collections;
pub use collections::user_rule_collection;

// pub mod rules_x;

pub trait Transaction {
  fn code(&mut self) -> &mut SqlCode;
}

pub trait WriteUpdates<Other = Self> {
  type Schema;

  fn write_updates(
    original: &Self, 
    modified: &Other,
    schema: &Self::Schema,
    modifications: &mut impl CompoundValueWriteDestination,
  );
}

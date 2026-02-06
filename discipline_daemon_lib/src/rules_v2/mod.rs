use std::collections::HashMap;
use crate::x::{TimeRange, UuidV4};

struct TimeRangeRule {
  enabler: (),
  activator: TimeRange,
}

struct TimeRangeRules {
  rules: HashMap<UuidV4, TimeRangeRule>,
}

struct AlwaysRule {
  enabler: (),
}

struct ScreenAllowanceRule {

}


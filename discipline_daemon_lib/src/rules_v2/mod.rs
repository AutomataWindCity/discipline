use std::collections::HashMap;
use crate::x::{TimeRange, UuidV4};

enum Protector {

}

struct TimeRangeRule {
  protector: Protector,
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


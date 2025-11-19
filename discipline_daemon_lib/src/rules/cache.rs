use std::collections::HashMap;
use crate::x::UuidV4;
use super::{RuleEnabler, RuleActivator};

pub struct CachedRule {
  enabler: RuleEnabler,
  activator: RuleActivator,
}

pub struct CachedRules {
  rules: HashMap<UuidV4, CachedRule>,
}
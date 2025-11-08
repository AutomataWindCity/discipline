use crate::x::TextualError;
use crate::x::rules::*;
use crate::x::database::*;


pub enum RuleActionConditionalType {
  Time,
  Always,
}

// impl RuleActivator
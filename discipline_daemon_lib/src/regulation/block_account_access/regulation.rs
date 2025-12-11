use serde::{Deserialize, Serialize};
use crate::x::rules;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
  pub rules: rules::RuleGroup,
}

impl Regulation {
  pub fn new() -> Self {
    Self {
      rules: rules::RuleGroup::new(),
    }
  }

  pub fn rules(&self) -> &rules::RuleGroup {
    &self.rules
  }

  pub fn rules_mut(&mut self) -> &mut rules::RuleGroup {
    &mut self.rules
  }
}

// pub mod procedures {
//   use serde::{Serialize, Deserialize};
//   use super::Regulation;
//   use crate::x::{Database, MonotonicInstant, UuidV4, rules};

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum AnyProcedure {
//     Rules(rules::procedures::Procedure),
//   }

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   pub enum AnyProcedureReturn {
//     Rules(rules::procedures::ProcedureReturn),
//   }

//   impl AnyProcedure {
//     pub async fn execute(
//       self,
//       now: MonotonicInstant,
//       user_id: UuidV4,
//       database: &Database,
//       regulation: &mut Regulation,
//       cross_rule_group_info: &mut rules::CrossGroupInfo,
//     ) -> AnyProcedureReturn {
//       match self {
//         AnyProcedure::Rules(inner) => {
//           AnyProcedureReturn::Rules(
//             inner
//               .execute(
//                 now, 
//                 &rules::Location::UserAccountAccessRegulation { user_id }, 
//                 database, 
//                 &mut regulation.rules, 
//                 cross_rule_group_info,
//               )
//               .await
//           )
//         }
//       }
//     }
//   }
// }
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlwaysConditional {
  __private: PhantomData<()>,
}

impl AlwaysConditional {
  pub fn new() -> Self {
    Self { __private: PhantomData }
  }

  pub fn evaulate(&self) -> bool {
    true
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  __private: PhantomData<()>
}

impl Creator {
  pub fn create(self) -> AlwaysConditional {
    AlwaysConditional::new()
  }
}

pub mod database {
  pub use crate::x::database::AlwaysConditionalSchema as Schema;
}
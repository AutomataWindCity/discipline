use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};
use super::Daemon;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum AnyProcedure {

}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum AnyProcedureReturn {}


impl AnyProcedure {
  pub async fn execute(self, daemon: &Daemon) -> AnyProcedureReturn {
    todo!()
  }
}
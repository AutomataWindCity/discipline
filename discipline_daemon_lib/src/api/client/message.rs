use serde::{Serialize, Deserialize};
use crate::x::procedures::Procedure;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
  ConnectionConfiguration(ClientConnectionConfiguration),
  CallProcedure(Procedure),
  CloseConnection(ClientConnectionCloseReason),
}

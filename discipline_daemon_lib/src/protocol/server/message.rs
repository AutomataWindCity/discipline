use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
  ConnectionConfiguration(ServerConnectionConfiguration),
  ProcedureReturn(Vec<u8>),
  Close(ServerCloseReason),
}

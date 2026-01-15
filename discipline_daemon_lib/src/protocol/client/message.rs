use serde::{Serialize, Deserialize};
use crate::x::procedures::Procedure;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EstablishConnection {
  WebUi,
  LinuxPamModule {
    password: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EstablishConnectionRef<'a> {
  WebUi,
  LinuxPamModule {
    password: &'a str,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloseConnection {
  Finished,
  InternalError,
  ProtocolError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
  EstablishConnection(EstablishConnection),
  CloseConnection(CloseConnection),
  CallProcedure(Procedure),
  SendNotification,
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerCloseReason {
  ServerInternalError,
  ServerBusy,
}

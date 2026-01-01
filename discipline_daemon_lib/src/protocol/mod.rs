mod magic_bytes;
use magic_bytes::PREFACE_MAGIC_BYTES;

mod serialization;
use serialization::*;

mod client;
pub use client::*;

mod server;
pub use server::*;

pub mod x;
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum ClientMessageType {
//   ConnectionConfiguration,
//   CallProcedure,
//   CloseConnection,
// }

// impl ClientMessageType {
//   const CONNECTION_CONFIGURATION_AS_NUMBER: u8 = 0;
//   const CALL_PROCEDURE_AS_NUMBER: u8 = 1;
//   const CLOSE_CONNECTION_AS_NUMBER: u8 = 2;

//   fn from_number(number: u8) -> Result<Self, TextualError> {
//     match number {
//       Self::CONNECTION_CONFIGURATION_AS_NUMBER => {
//         Ok(Self::ConnectionConfiguration)
//       }
//       Self::CALL_PROCEDURE_AS_NUMBER => {
//         Ok(Self::CallProcedure)
//       }
//       Self::CLOSE_CONNECTION_AS_NUMBER => {
//         Ok(Self::CloseConnection)
//       }
//       other => {
//         Err(
//           TextualError::new("Creating ClientMessageType from its numeric representation")
//             .with_message(format!("Invalid variant. Expected {} (for ConnectionConfiguration), {} (for CallProcedure) or {} (for CloseConnection), but found {}", Self::CONNECTION_CONFIGURATION_AS_NUMBER, Self::CALL_PROCEDURE_AS_NUMBER, Self::CLOSE_CONNECTION_AS_NUMBER, other))
//         )
//       }
//     }
//   }

//   fn to_number(&self) -> u8 {
//     match self {
//       Self::ConnectionConfiguration => {
//         Self::CONNECTION_CONFIGURATION_AS_NUMBER
//       }
//       Self::CallProcedure => {
//         Self::CALL_PROCEDURE_AS_NUMBER
//       }
//       Self::CloseConnection => {
//         Self::CLOSE_CONNECTION_AS_NUMBER
//       }
//     }
//   }
// }

// struct ClientMessageLength {
//   length: u32
// }

// impl ClientMessageLength {
//   fn new(length: u32) -> Self {
//     Self { length }
//   }

//   fn to_number(&self) -> u32 {
//     self.length
//   }

//   fn to_usize(&self) -> usize {
//     self.length as usize
//   }
// }

// struct ClientMessageHeader {
//   message_type: ClientMessageType,
//   message_length: ClientMessageLength,
// }
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// enum ServerMessageType {
//   ConnectionConfiguration,
//   ProcedureReturn,
//   CloseConnection,
// }

// impl ServerMessageType {
//   const CONNECTION_CONFIGURATION_AS_NUMBER: u8 = 0;
//   const PROCEDURE_RETURN_AS_NUMBER: u8 = 1;
//   const CLOSE_CONNECTION_AS_NUMBER: u8 = 2;

//   fn from_number(number: u8) -> Result<ServerMessageType, TextualError> {
//     match number {
//       Self::CONNECTION_CONFIGURATION_AS_NUMBER => {
//         Ok(Self::ConnectionConfiguration)
//       }
//       Self::PROCEDURE_RETURN_AS_NUMBER => {
//         Ok(Self::ProcedureReturn)
//       }
//       Self::CLOSE_CONNECTION_AS_NUMBER => {
//         Ok(Self::CloseConnection)
//       }
//       other => {
//         Err(
//           TextualError::new("Creating ServerMessageType from its numeric representation")
//             .with_message(format!("Invalid variant. Expected {} (for ConnectionConfiguration), {} (for ProcedureReturn) or {} (for CloseConnection) but found {}", Self::CONNECTION_CONFIGURATION_AS_NUMBER, Self::PROCEDURE_RETURN_AS_NUMBER, Self::CLOSE_CONNECTION_AS_NUMBER, other))
//         )
//       }
//     }
//   }

//   fn to_number(&self) -> u8 {
//     match self {
//       Self::ConnectionConfiguration => {
//         Self::CONNECTION_CONFIGURATION_AS_NUMBER
//       }
//       Self::ProcedureReturn => {
//         Self::PROCEDURE_RETURN_AS_NUMBER
//       }
//       Self::CloseConnection => {
//         Self::CLOSE_CONNECTION_AS_NUMBER
//       }
//     }
//   }
// }

// struct ServerMessageContentLength {
//   length: u32
// }

// impl ServerMessageContentLength {
//   fn from_usize(length: usize) -> Result<Self, ()> {
//     length
//       .try_into()
//       .map(|length| Self { length })
//       .map_err(|_| ())
//   }

//   fn new(length: u32) -> Self {
//     Self {
//       length
//     }
//   }
// }

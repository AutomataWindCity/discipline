use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use crate::operating_system;
use crate::x::{Duration, TextualError, protocol::*};

/// Connection established by the client to communicate 
/// with the server.
pub struct ClientConnectionSync {
  connection: TcpStream,
}

impl ClientConnectionSync {
  pub fn connect(
    address: SocketAddr, 
    timeout: Duration,
  ) -> Result<Self, TextualError> {
    let timeout = timeout.to_std_duration();

    let mut connection = TcpStream::connect_timeout(&address, timeout)
      .map_err(|error| {
        TextualError::new("Connecting to discipline server")
          .with_message("An io error occured when estabishing a TCP connection")
          .with_attachement_display("Server address", address)
          .with_attachement_display("Io error", error)
      })?;

    // TODO: Do proper error handling.
    connection.set_read_timeout(Some(timeout)).unwrap();
    // TODO: Do proper error handling.
    connection.set_write_timeout(Some(timeout)).unwrap();

    connection
      .write_all(&PREFACE_MAGIC_BYTES)
      .map_err(|error| {
        TextualError::new("Connecting to discipline server")
          .with_message("An io error occured when writing the connection preface")
          .with_attachement_display("Io error", error)
      })?;

    // TODO: Recieve a confirmation message

    return Ok(Self { 
      connection,
    });
  }

  pub fn read_message(&mut self) -> Result<ServerMessage, TextualError> {
    let mut content_length = [0; 4];

    self
      .connection
      .read_exact(&mut content_length)
      .map_err(|error| {
        TextualError::new("Client connection reading server message")
          .with_message("An io error occured while reading the message length")
          .with_attachement_display("Io error", error)
      })?;

    let content_length = u32::from_be_bytes(content_length);

    let mut message = Vec::with_capacity(content_length as usize);

    self
      .connection
      .read_exact(&mut message)
      .map_err(|error| {
        TextualError::new("Client connection reading server message")
          .with_message("An io error occured while reading the message content")
          .with_attachement_display("Io error", error)
      })?;

    deserialize(&message)
      .map_err(|error| {
        error
          .with_context("Client connection reading server message")
          .with_message("Failed to deserialize the message content")
      })
  }

  pub fn write_message(&mut self, message: &ClientMessage) -> Result<(), TextualError> {
    let content = serialize(message).map_err(|error| {
      error
        .with_context("Client connection writing server message")
        .with_message("Failed to serialize the server message")
    })?;
    
    let content_length: u32 = content.len().try_into().map_err(|error| {
      TextualError::new("Client connection writing server message")
        .with_message("Message content length is too large to fit in u32")
        .with_attachement_display("Cast error", error)
        .with_attachement_debug("Message", message)
        .with_attachement_display("Message content length", content.len())
    })?;

    self
      .connection
      .write_all(&content_length.to_be_bytes())
      .map_err(|error| {
        TextualError::new("Client connection writing server message")
          .with_message("An io error occured while writing the message content length")
          .with_attachement_display("Io error", error)
      })?;

    self
      .connection
      .write_all(&content)
      .map_err(|error| {
        TextualError::new("Client connection writing server message")
          .with_message("An io error occured while writing the message content")
          .with_attachement_display("Io error", error)
      })?;

    Ok(())
  }
}

// async fn respond_to_incoming_message(
//   daemon: &Daemon,
//   connection: &mut ClientConnection,
// ) -> Result<(), TextualError> {
//   let message = match connection.read_message().await {
//     Ok(value) => {
//       value
//     }
//     Err(error) => {
//       return Err(
//         error
//           .with_context("Handling incoming client message")
//           .with_message("An error occured while reading the client message")
//       );
//     }
//   };

//   let procedure = match message {
//     ClientMessage::CallProcedure(procedure) => {
//       procedure
//     }
//     ClientMessage::CloseConnection(_) => {
//       return Ok(());
//     }
//     ClientMessage::ConnectionConfiguration(_) => {
//       return Err(
//         TextualError::new("Handling incoming client message")
//           .with_message("The client sent a Connection Configuration Message. It is unexpected at this stage. A Connection Configuration Message is sent by the client one time after the handshake is complete. The server was expecting either a Procedure or Close Connection Message.")
//       );
//     }
//   };

//   match_procedure!(procedure => {
//     let return_value = procedure.execute(daemon).await;

//     let return_value_serialized = match serialize(&return_value) {
//       Ok(value) => {
//         value
//       }
//       Err(error) => {
//         return Err(
//           error
//             .with_context("Handling incoming client message")
//             .with_message("The server recieved a Procedure message from the client. The server executed the specified procedure. But an error occured while serializing the procedure's return to be sent to the client.")
//             .with_attachement_debug("Procedure return", return_value)
//         );
//       }
//     };

//     connection
//       .write_message(&ServerMessage::ProcedureReturn(return_value_serialized))
//       .await
//       .map_err(|error| {
//         error
//           .with_context("Handling incoming client message")
//           .with_message("The server recieved a Procedure message from the client. The server executed the specified procedure and serialized its return value, but an error occured while sending the procedure's return value to the client.")
//           .with_attachement_debug("Procedure return", return_value)
//       })
//   })
// }

// async fn respond_to_incoming_messages(
//   daemon: Arc<Daemon>,
//   permit: OwnedSemaphorePermit,
//   mut connection: ClientConnection,
// ) {
//   loop {
//     let result = respond_to_incoming_message(&daemon, &mut connection).await;

//     match result {
//       Ok(()) => {
//         continue;
//       }
//       Err(error) => {
//         eprintln!("{error}");
//         break;
//       }
//     }
//   }

//   drop(permit);
// }

// impl ClientConnection {
//   pub fn auto_serve(
//     self, 
//     daemon: Arc<Daemon>,
//     permit: OwnedSemaphorePermit,
//   ) -> JoinHandle<()> {
//     spawn(async {
//       respond_to_incoming_messages(daemon, permit, self).await;
//     })
//   }
// }

impl ClientConnectionSync {
  pub fn is_user_account_access_blocked(
    &mut self, 
    operating_system_user_name: &operating_system::UserName,
  ) -> Result<bool, TextualError> {
    todo!()
  }

  pub fn notify_that_user_session_opened(
    &mut self, 
    operating_system_user_name: &operating_system::UserName,
  ) -> Result<(), TextualError> {
    todo!()
  }

  pub fn notify_that_user_session_closed(
    &mut self, 
    operating_system_user_name: &operating_system::UserName,
  ) -> Result<(), TextualError> {
    todo!()
  }
}
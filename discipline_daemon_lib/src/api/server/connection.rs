use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::task::JoinHandle;
use crate::x::procedures::Procedure;
use crate::x::{Daemon, TextualError, match_procedure, api::*};

pub struct ServerConnection {
  connection: TcpStream,
  is_closed: bool,
}

impl ServerConnection {
  pub async fn handshake(mut connection: TcpStream) -> Result<Self, TextualError> {
    let mut buffer = [0u8; PREFACE_MAGIC_BYTES.len()];
    
    loop {
      connection
        .read_exact(&mut buffer)
        .await 
        .map_err(|error| {
          TextualError::new("Performing ServerConnection handshake")
            .with_message("An io error occured while reading the protocol preface")
            .with_attachement_display("Io error", error)
        })?;

      if buffer != PREFACE_MAGIC_BYTES {
        return Err(
          TextualError::new("Performing ServerConnection handshake")
            .with_message("First data recieved on the connection wasn't the protocol preface")
            .with_attachement_debug("Recieved data", buffer)
            .with_attachement_debug("Connection preface", &PREFACE_MAGIC_BYTES)
        );
      }

      return Ok(Self { 
        connection,
        is_closed: false,
      });
    }
  }

  async fn read_message(&mut self) -> Result<ClientMessage, TextualError> {
    let mut content_length = [0; 4];

    self
      .connection
      .read_exact(&mut content_length)
      .await
      .map_err(|error| {
        TextualError::new("Reading client message")
          .with_message("An io error occured while reading the message length")
          .with_attachement_display("Io error", error)
      })?;

    let content_length = u32::from_be_bytes(content_length);

    let mut message = Vec::with_capacity(content_length as usize);

    self
      .connection
      .read_exact(&mut message)
      .await
      .map_err(|error| {
        TextualError::new("Reading client message")
          .with_message("An io error occured while reading the message content")
          .with_attachement_display("Io error", error)
      })?;

    bincode_deserialize(&message)
      .map_err(|error| {
        error
          .with_context("Reading client message")
          .with_message("Failed to deserialize the message content")
      })
  }

  async fn write_message(&mut self, message: &ServerMessage) -> Result<(), TextualError> {
    let content = bincode_serialize(message).map_err(|error| {
      error
        .with_context("Writing server message")
        .with_message("Failed to serialize the server message")
    })?;
    
    let content_length: u32 = content.len().try_into().map_err(|error| {
      TextualError::new("Writing server message")
        .with_message("Message content length is too large to fit in u32")
        .with_attachement_display("Cast error", error)
        .with_attachement_debug("Message", message)
        .with_attachement_display("Message content length", content.len())
    })?;

    self
      .connection
      .write_all(&content_length.to_be_bytes())
      .await
      .map_err(|error| {
        TextualError::new("Writing server message")
          .with_message("An io error occured while writing the message content length")
          .with_attachement_display("Io error", error)
      })?;

    self
      .connection
      .write_all(&content)
      .await
      .map_err(|error| {
        TextualError::new("Writing server message")
          .with_message("An io error occured while writing the message content")
          .with_attachement_display("Io error", error)
      })?;

    Ok(())
  }
}

async fn respond_to_incoming_message(
  daemon: &Daemon,
  connection: &mut ServerConnection,
) -> Result<(), TextualError> {
  let message = match connection.read_message().await {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(
        error
          .with_context("Handling incoming client message")
          .with_message("An error occured while reading the client message")
      );
    }
  };

  let procedure = match message {
    ClientMessage::CallProcedure(procedure) => {
      procedure
    }
    ClientMessage::CloseConnection(_) => {
      return Ok(());
    }
    ClientMessage::ConnectionConfiguration(_) => {
      return Err(
        TextualError::new("Handling incoming client message")
          .with_message("The client sent a Connection Configuration Message. It is unexpected at this stage. A Connection Configuration Message is sent by the client one time after the handshake is complete. The server was expecting either a Procedure or Close Connection Message.")
      );
    }
  };

  match_procedure!(procedure => {
    let return_value = procedure.execute(daemon).await;

    let return_value_serialized = match bincode_serialize(&return_value) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(
          error
            .with_context("Handling incoming client message")
            .with_message("The server recieved a Procedure message from the client. The server executed the specified procedure. But an error occured while serializing the procedure's return to be sent to the client.")
            .with_attachement_debug("Procedure return", return_value)
        );
      }
    };

    connection
      .write_message(&ServerMessage::ProcedureReturn(return_value_serialized))
      .await
      .map_err(|error| {
        error
          .with_context("Handling incoming client message")
          .with_message("The server recieved a Procedure message from the client. The server executed the specified procedure and serialized its return value, but an error occured while sending the procedure's return value to the client.")
          .with_attachement_debug("Procedure return", return_value)
      })
  })
}

async fn respond_to_incoming_messages(
  daemon: Arc<Daemon>,
  permit: OwnedSemaphorePermit,
  mut connection: ServerConnection,
) {
  loop {
    let result = respond_to_incoming_message(&daemon, &mut connection).await;

    match result {
      Ok(()) => {
        continue;
      }
      Err(error) => {
        eprintln!("{error}");
        break;
      }
    }
  }

  drop(permit);
}

impl ServerConnection {
  pub fn auto_serve(
    self, 
    daemon: Arc<Daemon>,
    permit: OwnedSemaphorePermit,
  ) -> JoinHandle<()> {
    spawn(async {
      respond_to_incoming_messages(daemon, permit, self).await;
    })
  }
}
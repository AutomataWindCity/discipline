use std::any::type_name;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use bincode::{Encode, Decode};
use tokio::spawn;
use tokio::sync::Mutex;
use crate::daemon::procedures::{AnyProcedure, AnyProcedureReturn};
use crate::x::{Daemon, TextualError};

pub enum Error {
  CreateServerIoError,
  ShutdownError,
}

trait MyIo {
  async fn my_shutdown(&mut self);
}

impl<T> MyIo for T 
where
  T: Unpin + AsyncRead + AsyncWrite
{
  async fn my_shutdown(&mut self) {
    let _ = self.shutdown().await;
  }
}

static PREFACE_MAGIC_BYTES: [u8; 16] = [
  0xDE, 0xAD, 0xBE, 0xEF,
  0xCA, 0xFE, 0xBA, 0xBE,
  0x0D, 0x15, 0xEA, 0x5E,
  0x50, 0x52, 0x30, 0x54
];

static BINCODE_CONFIG: bincode::config::Configuration<
  bincode::config::BigEndian,
  bincode::config::Fixint,
  bincode::config::Limit<500>
> = bincode::config::standard()
  .with_big_endian()
  .with_fixed_int_encoding()
  .with_limit();

fn bincode_serialize<T>(value: &T) -> Result<Vec<u8>, TextualError> 
where 
  T: Encode
{
  bincode::encode_to_vec(value, BINCODE_CONFIG)
    .map_err(|error| {
      TextualError::new(format!("Serializing {} using bincode", type_name::<T>()))
        .with_attachement_display("Error", error)
    })
}

fn bincode_deserialize<T>(slice: &[u8]) -> Result<T, TextualError>
where 
  T: Decode<()>
{
  let (value, read_bytes) = match bincode::decode_from_slice(slice, BINCODE_CONFIG) {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(
        TextualError::new(format!("Deserializing message content as {} using bincode", type_name::<T>()))
          .with_attachement_display("Error", error)
      );
    }
  };

  if read_bytes != slice.len() {
    return Err(
      TextualError::new(format!("Deserializing message content as {} using bincode", type_name::<T>()))
        .with_message("Bincode said it deserialized the message content successfully, but the number of bytes bincode read is not the same as the message content length, which means the message is invalid because it contains additional data or its length is wrong.")
        .with_attachement_display("Message content length", slice.len())
        .with_attachement_debug("Message content", slice)
    )
  }
  
  Ok(value)
}

#[derive(Debug, Clone, Encode, Decode)]
struct ClientConnectionConfiguration {

}

#[derive(Debug, Clone, Encode, Decode)]
enum ClientConnectionCloseReason {
  Finished,
  InternalError,
}

#[derive(Debug, Clone, Encode, Decode)]
enum ClientMessage {
  ConnectionConfiguration(ClientConnectionConfiguration),
  CallProcedure(AnyProcedure),
  CloseConnection(ClientConnectionCloseReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientMessageType {
  ConnectionConfiguration,
  CallProcedure,
  CloseConnection,
}

impl ClientMessageType {
  const CONNECTION_CONFIGURATION_AS_NUMBER: u8 = 0;
  const CALL_PROCEDURE_AS_NUMBER: u8 = 1;
  const CLOSE_CONNECTION_AS_NUMBER: u8 = 2;

  fn from_number(number: u8) -> Result<Self, TextualError> {
    match number {
      Self::CONNECTION_CONFIGURATION_AS_NUMBER => {
        Ok(Self::ConnectionConfiguration)
      }
      Self::CALL_PROCEDURE_AS_NUMBER => {
        Ok(Self::CallProcedure)
      }
      Self::CLOSE_CONNECTION_AS_NUMBER => {
        Ok(Self::CloseConnection)
      }
      other => {
        Err(
          TextualError::new("Creating ClientMessageType from its numeric representation")
            .with_message(format!("Invalid variant. Expected {} (for ConnectionConfiguration), {} (for CallProcedure) or {} (for CloseConnection), but found {}", Self::CONNECTION_CONFIGURATION_AS_NUMBER, Self::CALL_PROCEDURE_AS_NUMBER, Self::CLOSE_CONNECTION_AS_NUMBER, other))
        )
      }
    }
  }

  fn to_number(&self) -> u8 {
    match self {
      Self::ConnectionConfiguration => {
        Self::CONNECTION_CONFIGURATION_AS_NUMBER
      }
      Self::CallProcedure => {
        Self::CALL_PROCEDURE_AS_NUMBER
      }
      Self::CloseConnection => {
        Self::CLOSE_CONNECTION_AS_NUMBER
      }
    }
  }
}

struct ClientMessageLength {
  length: u32
}

impl ClientMessageLength {
  fn new(length: u32) -> Self {
    Self { length }
  }

  fn to_number(&self) -> u32 {
    self.length
  }

  fn to_usize(&self) -> usize {
    self.length as usize
  }
}

struct ClientMessageHeader {
  message_type: ClientMessageType,
  message_length: ClientMessageLength,
}

#[derive(Debug, Clone, Encode, Decode)]
struct ServerConnectionConfiguration {

}

#[derive(Debug, Clone, Encode, Decode)]
enum ServerCloseReason {
  ServerInternalError,
  ServerBusy,
}

#[derive(Debug, Clone, Encode, Decode)]
enum ServerMessage {
  ConnectionConfiguration(ServerConnectionConfiguration),
  ProcedureReturn(AnyProcedureReturn),
  Close(ServerCloseReason),
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
enum ServerMessageType {
  ConnectionConfiguration,
  ProcedureReturn,
  CloseConnection,
}

impl ServerMessageType {
  const CONNECTION_CONFIGURATION_AS_NUMBER: u8 = 0;
  const PROCEDURE_RETURN_AS_NUMBER: u8 = 1;
  const CLOSE_CONNECTION_AS_NUMBER: u8 = 2;

  fn from_number(number: u8) -> Result<ServerMessageType, TextualError> {
    match number {
      Self::CONNECTION_CONFIGURATION_AS_NUMBER => {
        Ok(Self::ConnectionConfiguration)
      }
      Self::PROCEDURE_RETURN_AS_NUMBER => {
        Ok(Self::ProcedureReturn)
      }
      Self::CLOSE_CONNECTION_AS_NUMBER => {
        Ok(Self::CloseConnection)
      }
      other => {
        Err(
          TextualError::new("Creating ServerMessageType from its numeric representation")
            .with_message(format!("Invalid variant. Expected {} (for ConnectionConfiguration), {} (for ProcedureReturn) or {} (for CloseConnection) but found {}", Self::CONNECTION_CONFIGURATION_AS_NUMBER, Self::PROCEDURE_RETURN_AS_NUMBER, Self::CLOSE_CONNECTION_AS_NUMBER, other))
        )
      }
    }
  }

  fn to_number(&self) -> u8 {
    match self {
      Self::ConnectionConfiguration => {
        Self::CONNECTION_CONFIGURATION_AS_NUMBER
      }
      Self::ProcedureReturn => {
        Self::PROCEDURE_RETURN_AS_NUMBER
      }
      Self::CloseConnection => {
        Self::CLOSE_CONNECTION_AS_NUMBER
      }
    }
  }
}

struct ServerMessageContentLength {
  length: u32
}

impl ServerMessageContentLength {
  fn from_usize(length: usize) -> Result<Self, ()> {
    length
      .try_into()
      .map(|length| Self { length })
      .map_err(|_| ())
  }

  fn new(length: u32) -> Self {
    Self {
      length
    }
  }
}

struct ServerConnection {
  connection: TcpStream,
  is_closed: bool,
}

impl ServerConnection {
  async fn handshake(mut connection: TcpStream) -> Result<Self, TextualError> {
    let mut buffer: [u8; 16] = [0; 16];
    
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

  async fn task(
    mut self, 
    permit: OwnedSemaphorePermit, 
    daemon: Arc<Daemon>,
  ) {
    loop {
      let Ok(message) = self.read_message().await else {
        return;
      };

      let procedure = match message {
        ClientMessage::CallProcedure(procedure) => {
          procedure
        }
        ClientMessage::CloseConnection(reason) => {
          return;
        }
        ClientMessage::ConnectionConfiguration(_) => {
          return;
        }
      };

      let procedure_return = procedure.execute(&daemon).await;

      if let Err(error) = self.write_message(&ServerMessage::ProcedureReturn(procedure_return)).await {
        return;
      }
    }
  }

  fn launch_task(
    self,
    permit: OwnedSemaphorePermit,
    daemon: Arc<Daemon>,
  ) {
    spawn(async {
      self.task(permit, daemon)
    });
  }
}

struct ServerInner {
  tcp_listener: TcpListener,
  status: ServerStatus,
  semaphore: Arc<Semaphore>,
}

pub enum CreateServerError {
  BindError(std::io::Error),
}

#[derive(Debug)]
pub enum AcceptConnectionError {
  TcpListenerAccept(std::io::Error)
}

enum ServerStatus {
  Started,
  Stopped,
}

impl ServerStatus {
  fn is_started(&self) -> bool {
    matches!(self, Self::Started)
  }
  fn is_stopped(&self) -> bool {
    matches!(self, Self::Stopped)
  }
}

impl ServerInner {
  pub async fn new(address: impl ToSocketAddrs) -> Result<Self, CreateServerError> {
    let tcp_listener = match TcpListener::bind(address).await {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(CreateServerError::BindError(error));
      }
    };

    Ok(Self { 
      tcp_listener, 
      status: ServerStatus::Stopped, 
      semaphore: Arc::new(Semaphore::const_new(6))
    })
  }

  async fn accept_connection(&self) -> Result<ServerConnection, TextualError> {
    let connection = self.tcp_listener.accept().await
      .map(|value| {
        value.0
      })
      .map_err(|error| {
        TextualError::new("Accepting a connection from client")
          .with_message("An io error occured while calling 'accept' on the tcp listener")
          .with_attachement_display("Io error", error)
      })?;

    ServerConnection::handshake(connection)
      .await
      .map_err(|error| {
        error
          .with_context("Accepting a connection from client")
      })
  }
}

pub struct Server {
  server: Arc<Mutex<ServerInner>>,
}

impl Server {
  async fn stop(self) {
    self.server.lock().await.status = ServerStatus::Stopped;
  }

  pub async fn start(self, daemon: Arc<Daemon>) -> Result<(), ()> {
    let mut server = self.server.lock().await;
    if server.status.is_started() {
      return Err(());
    } else {
      server.status = ServerStatus::Started;
      drop(server);
    }

    let server = Arc::clone(&self.server);
    loop {
      let server = server.lock().await;
      let semaphore = Arc::clone(&server.semaphore);
      
      let Ok(permit) = semaphore.acquire_owned().await else {
        return Err(());
      };

      let Ok(connection) = server.accept_connection().await else {
        continue;
      };

      connection.launch_task(permit, Arc::clone(&daemon));
    }
  }
}
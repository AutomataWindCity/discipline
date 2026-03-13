use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::OwnedSemaphorePermit;
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, Semaphore};

use serde::{Serialize, Deserialize};

use crate::x::{IsTextualError, OptionalTextualErrorContext};

struct Daemon {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Procedure {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProcedureReturn {}

impl Procedure {
  fn execute(
    self, 
    daemon: &Daemon, 
    textual_error: &mut impl IsTextualError,
  ) -> Result<ProcedureReturn, ()> {
    todo!()
  }
}

impl Serializable for Procedure {
  
}

impl Deserializable for Procedure {
  
}

impl Serializable for ProcedureReturn {
  
}

impl Deserializable for ProcedureReturn {
  
}

pub static MAGIC_BYTES: [u8; 16] = [
  0xDE, 0xAD, 0xBE, 0xEF,
  0xCA, 0xFE, 0xBA, 0xBE,
  0x0D, 0x15, 0xEA, 0x5E,
  0x50, 0x52, 0x30, 0x54
];

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EstablishConnection {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EstablishConnectionReply {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloseConnection {
  Finished,
  InternalError,
  ProtocolError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
  CloseConnection(CloseConnection),
  CallProcedure(Procedure),
  // SendNotification,
}

pub trait Serializable {}

pub trait Deserializable: Sized {}

pub trait Serializer {
  fn serialize(
    &self, 
    value: &impl Serializable, 
    buffer: &mut [u8],
    textual_error: &mut impl IsTextualError,
  ) -> Result<usize, ()>;

  fn deserialize<T>(
    &self, 
    buffer: &[u8], 
    textual_error: &mut impl IsTextualError,
  ) -> Result<T, ()>
  where
    T: Deserializable;
}

// Maybe rename to Datagram.
const MESSAGE_LENGTH_SIZE: usize = size_of::<u32>();

pub struct Stream<Ser: Serializer> {
  stream: TcpStream,
  serialization: Ser,
  buffer: Vec<u8>
}

impl<Ser: Serializer> Stream<Ser> {
  pub fn construct(
    stream: TcpStream,
    serializer: Ser,
    maximum_message_length: usize,
  ) -> Self {
    Self { 
      stream,
      // TODO: Handle addition error
      buffer: vec![0; MESSAGE_LENGTH_SIZE + maximum_message_length],
      serialization: serializer,
    }
  }

  async fn read<Message: Deserializable>(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Message, ()> {
    let mut textual_error = textual_error.optional_context("Stream reading a messaage message from peer");

    let mut content_length = [0; 4];

    self
      .stream
      .read_exact(&mut content_length)
      .await
      .map_err(|error| {
        textual_error.change_context("Reading message length");
        textual_error.add_attachement_display("Io error", error);
        ()
      })?;

    let content_length = u32::from_be_bytes(content_length);

    let mut message = Vec::with_capacity(content_length as usize);

    self
      .stream
      .read_exact(&mut message)
      .await
      .map_err(|error| {
        textual_error.change_context("Reading message content");
        textual_error.add_attachement_display("Io error", error);
        ()
      })?;

    self
      .serialization
      .deserialize(&message, &mut textual_error.optional_context("Deserializing message"))
  }

  async fn write<Message: Serializable>(
    &mut self, 
    message: &Message,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error.optional_context("Sending a message to peer");

    let message_buffer = &mut self.buffer[MESSAGE_LENGTH_SIZE..];
    let message_length = self
      .serialization
      .serialize(
        message, 
        message_buffer, 
        &mut textual_error.optional_context("Serializing message"),
      )?;
    
    let message_length_as_u32: u32 = message_length
      .try_into()
      .map_err(|error| {
        textual_error.add_message("Message length returned by the Serialization cannot be cast to u32 maybe because it's larger than maximum supported message length");
        textual_error.add_attachement_display("Cast error", error)
      })?;

    let message_length_buffer = &mut self.buffer[0..MESSAGE_LENGTH_SIZE];
    message_length_buffer.copy_from_slice(&message_length_as_u32.to_be_bytes());

    let buffer = &self.buffer[..MESSAGE_LENGTH_SIZE + message_length];

    self
      .stream
      .write_all(buffer)
      .await
      .map_err(|error| {
        textual_error.add_message("An io error occured while sending the message");
        textual_error.add_attachement_display("Io error", error);
      })?;

    Ok(())
  }
}

pub struct ClientStream<Ser: Serializer> {
  stream: Stream<Ser>,
}

impl<Ser: Serializer> ClientStream<Ser> {
  pub fn construct(stream: Stream<Ser>) -> Self {
    Self { stream }
  }

  pub async fn write_magic_bytes(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    todo!()
  }

  pub async fn write_establish_connection(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    todo!()
  }

  pub async fn read_establish_connection_reply(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<EstablishConnectionReply, ()> {
    todo!()
  }
}

/// Connection established by the client to communicate 
/// with the server.
pub struct ClientConnection<Ser: Serializer> {
  stream: ClientStream<Ser>,
}

impl<Ser: Serializer> ClientConnection<Ser> {
  pub fn construct(stream: ClientStream<Ser>) -> Self {
    Self { stream }
  }

  pub async fn connect(
    server_address: SocketAddr,
    serializer: Ser,
    maximum_message_length: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error.optional_context("Discipline API ClientConnection connecting to Discipline API Server");

    let stream = TcpStream::connect(&server_address)
      .await
      .map_err(|error| {
        textual_error.add_message("Failed to connect to the server over TCP");
        textual_error.add_attachement_display("Server address", server_address);
        textual_error.add_attachement_display("Io error", error);
      })?;

    let stream = Stream::construct(
      stream, 
      serializer, 
      maximum_message_length,
    );

    let mut stream = ClientStream::construct(stream);

    stream
      .write_establish_connection(&mut textual_error)
      .await?;

    let _reply = stream
      .read_establish_connection_reply(&mut textual_error)
      .await?;

    Ok(Self { stream })
  }
}

pub struct ServerStream<Ser: Serializer> {
  stream: Stream<Ser>,
}

impl<Ser: Serializer> ServerStream<Ser> {
  pub fn construct(stream: Stream<Ser>) -> Self {
    Self { 
      stream,
    }
  }

  pub async fn recv_establish_connection(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<EstablishConnection, ()> {
    todo!()
  }

  pub async fn send_establish_connection_reply(
    &mut self,
    reply: &EstablishConnectionReply,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    todo!()
  }

  pub async fn recv_message(
    &mut self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<ClientMessage, ()> {
    todo!()
  }

  pub async fn send_message(
    &mut self,
    message: &ClientMessage,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    todo!()
  }

  pub async fn send_procedure_return(
    &mut self,
    procedure_return: &ProcedureReturn,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    todo!()
  }
  // async fn read_message(&mut self) -> Result<ClientMessage, TextualError> {
  //   let mut content_length = [0; 4];

  //   self
  //     .stream
  //     .read_exact(&mut content_length)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Reading client message")
  //         .with_message("An io error occured while reading the message length")
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   let content_length = u32::from_be_bytes(content_length);

  //   let mut message = Vec::with_capacity(content_length as usize);

  //   self
  //     .stream
  //     .read_exact(&mut message)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Reading client message")
  //         .with_message("An io error occured while reading the message content")
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   deserialize(&message)
  //     .map_err(|error| {
  //       error
  //         .with_context("Reading client message")
  //         .with_message("Failed to deserialize the message content")
  //     })
  // }

  // async fn write_message(&mut self, message: &ServerMessage) -> Result<(), TextualError> {
  //   let content = serialize(message).map_err(|error| {
  //     error
  //       .with_context("Writing server message")
  //       .with_message("Failed to serialize the server message")
  //   })?;
    
  //   let content_length: u32 = content.len().try_into().map_err(|error| {
  //     TextualError::new("Writing server message")
  //       .with_message("Message content length is too large to fit in u32")
  //       .with_attachement_display("Cast error", error)
  //       .with_attachement_debug("Message", message)
  //       .with_attachement_display("Message content length", content.len())
  //   })?;

  //   self
  //     .stream
  //     .write_all(&content_length.to_be_bytes())
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Writing server message")
  //         .with_message("An io error occured while writing the message content length")
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   self
  //     .stream
  //     .write_all(&content)
  //     .await
  //     .map_err(|error| {
  //       TextualError::new("Writing server message")
  //         .with_message("An io error occured while writing the message content")
  //         .with_attachement_display("Io error", error)
  //     })?;

  //   Ok(())
  // }
}

pub struct ServerConnection<Ser: Serializer> {
  stream: ServerStream<Ser>,
  is_closed: bool,
}

impl<Ser> ServerConnection<Ser> 
where 
  Ser: Serializer + Send + 'static
{
  pub async fn handshake(
    stream: TcpStream,
    serializer: Ser,
    maximum_message_length: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error.optional_context("Discipline API Server Connection performing handshake with client");

    let mut stream = ServerStream::construct(Stream::construct(
      stream, 
      serializer, 
      maximum_message_length,
    ));
    
    let _establish_connection = stream
      .recv_establish_connection(&mut textual_error)
      .await?;

    stream
      .send_establish_connection_reply(
        &EstablishConnectionReply {  }, 
        &mut textual_error
      )
      .await?;

    Ok(Self { 
      stream,
      is_closed: false,
    })
    // let mut buffer = [0u8; PREFACE_MAGIC_BYTES.len()];
    
    // loop {
    //   stream
    //     .read_exact(&mut buffer)
    //     .await 
    //     .map_err(|error| {
    //       TextualError::new("Performing ServerConnection handshake")
    //         .with_message("An io error occured while reading the protocol preface")
    //         .with_attachement_display("Io error", error)
    //     })?;

    //   if buffer != PREFACE_MAGIC_BYTES {
    //     return Err(
    //       TextualError::new("Performing ServerConnection handshake")
    //         .with_message("First data recieved on the connection wasn't the protocol preface")
    //         .with_attachement_debug("Recieved data", buffer)
    //         .with_attachement_debug("Connection preface", &PREFACE_MAGIC_BYTES)
    //     );
    //   }

    //   return Ok(Self { 
    //     stream,
    //   });
    // }
  }

  async fn process(
    &mut self, 
    daemon: &Daemon,
    textual_error: &mut impl IsTextualError,
  ) -> Result<(), ()> {
    let mut textual_error = textual_error.optional_context("Discipline API Server Connection receiving and processing a message from client");

    let message = self
      .stream
      .recv_message(&mut textual_error.optional_context("Receiving message "))
      .await
      .map_err(|_| {
        self.is_closed = true;
      })?;

    let procedure = match message {
      ClientMessage::CallProcedure(procedure) => {
        procedure
      }
      ClientMessage::CloseConnection(_) => {
        self.is_closed = true;
        return Ok(());
      }
    };

    let procedure_return = procedure.execute(
      daemon, 
      &mut textual_error,
    )?;

    self
      .stream
      .send_procedure_return(
        &procedure_return, 
        &mut textual_error,
      )
      .await
      .map_err(|_| {
        self.is_closed = true;
      })
  }

  async fn start_auto_processing(
    mut self,
    permit: OwnedSemaphorePermit,
    daemon: Arc<Daemon>,
  ) {
    let mut textual_error = OptionalTextualErrorContext::new("Discipline API Server Connection automatically accepting and processing incoming messages");

    loop {
      if self.is_closed {
        drop(permit);
        return;
      }

      if let Err(()) = self.process(&daemon, &mut textual_error).await {
        self.is_closed = true;
        drop(permit);
        return;
      }
    }
  }

  pub fn launch_auto_processing_loop(
    self, 
    daemon: Arc<Daemon>,
    permit: OwnedSemaphorePermit,
  ) -> JoinHandle<()> {
    spawn(async move {
      self.start_auto_processing(permit, daemon).await;
    })
  }
} 

#[derive(Debug)]
enum ServerStatus {
  Started,
  Stopped,
}

impl ServerStatus {
  fn is_started(&self) -> bool {
    matches!(self, Self::Started)
  }

  // fn is_stopped(&self) -> bool {
  //   matches!(self, Self::Stopped)
  // }
}

struct ServerSharedData<Ser: Serializer + Copy + Sync + Send + 'static> {
  tcp_listener: TcpListener,
  status: ServerStatus,
  semaphore: Arc<Semaphore>,
  serializer: Ser,
  maximum_message_length: usize,
}

#[derive(Debug)]
pub enum CreateServerError {
  BindError(std::io::Error),
}

#[derive(Debug)]
pub enum AcceptConnectionError {
  TcpListenerAccept(std::io::Error)
}

impl<Ser: Serializer + Copy + Sync + Send + 'static> ServerSharedData<Ser> {
  async fn accept(
    &self,
    textual_error: &mut impl IsTextualError,
  ) -> Result<ServerConnection<Ser>, ()> {
    let mut textual_error = textual_error.optional_context("Discipline API Server accepting a connection");

    let stream = self
      .tcp_listener
      .accept()
      .await
      .map(|value| {
        value.0
      })
      .map_err(|error| {
        textual_error.add_attachement_display("Io error", error)
      })?;

    ServerConnection
      ::handshake(
        stream,
        self.serializer,
        self.maximum_message_length,
        &mut textual_error,
      )
      .await
  }
}

pub struct Server<Ser: Serializer + Copy + Sync + Send + 'static> {
  server: Arc<Mutex<ServerSharedData<Ser>>>,
}

impl<Ser: Serializer + Copy + Sync + Send + 'static> Server<Ser> {
  pub async fn new(
    port: u16, 
    serializer: Ser,
    maximum_message_length: usize,
    maximum_concurrent_connections: usize,
    textual_error: &mut impl IsTextualError,
  ) -> Result<Self, ()> {
    let mut textual_error = textual_error.optional_context("Creating Discipline API Server");

    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    
    let tcp_listener = TcpListener::bind(&address)
      .await
      .map_err(|error| {
        textual_error.add_message("Failed to bind the TcpListener");
        textual_error.add_attachement_display("Server address", address);
        textual_error.add_attachement_display("Io error", error);
      })?;

    Ok(Self {
      server: Arc::new(Mutex::new(ServerSharedData { 
        status: ServerStatus::Stopped, 
        serializer,
        tcp_listener, 
        semaphore: Arc::new(Semaphore::const_new(maximum_concurrent_connections)),
        maximum_message_length,
      }))
    })
  }

  pub async fn start_auto_serving(&self, daemon: Arc<Daemon>) -> Result<(), ()> {
    let mut textual_error = OptionalTextualErrorContext::new("Starting Discipline Api Server's auto serving loop");

    let mut server = self.server.lock().await;
    if server.status.is_started() {
      textual_error.add_message("Loop has already started");
      return Err(());
    } else {
      server.status = ServerStatus::Started;
      drop(server);
    }

    let server = Arc::clone(&self.server);
    loop {
      let mut textual_error = OptionalTextualErrorContext::new("Discipline API Server auto serving loop is waiting to receive a new connection");

      let server = server.lock().await;
      let semaphore = Arc::clone(&server.semaphore);
      
      let Ok(permit) = semaphore.acquire_owned().await else {
        textual_error.add_message("Failed to aquire a semaphore permit. This a fatal internal error.");
        return Err(());
      };

      let Ok(stream) = server.accept(&mut textual_error).await else {
        continue;
      };

      stream.launch_auto_processing_loop(
        Arc::clone(&daemon), 
        permit,
      );
    }
  }

  // pub async fn stop_auto_serving(self) {
  //   // TODO: It will take way too long to aquire a lock due to
  //   // the "server.accept_connection().await" in "start_auto_serving".
  //   // This is not a problem now because "stop_auto_serving" is never
  //   // called anywhere yet. 
  //   self.server.lock().await.status = ServerStatus::Stopped;
  // }
}
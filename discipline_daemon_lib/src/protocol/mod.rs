mod http;

use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream, ToSocketAddrs }};
use crate::x::TextualError;

static PREFACE_MAGIC_BYTES: [u8; 16] = [
  0xDE, 0xAD, 0xBE, 0xEF,
  0xCA, 0xFE, 0xBA, 0xBE,
  0x0D, 0x15, 0xEA, 0x5E,
  0x50, 0x52, 0x30, 0x54
];

pub struct ClientConnectionConfiguration {

}

pub enum ClientMessage {
  ConnectionConfiguration(ClientConnectionConfiguration),
  CallProcedure,
  CloseConnection,
}

pub enum ServerCloseReason {
  ServerInternalError,
  ServerBusy,
}

pub enum ServerMessage {
  ConnectionConfigurationAknowlagded,
  ProcedureReturn,
  Close(ServerCloseReason),
}

pub struct Server {
  tcp_listener: TcpListener,
}

pub enum CreateServerError {
  BindError,
}

impl Server {
  pub async fn new(address: impl ToSocketAddrs) -> Result<Self, CreateServerError> {
    let tcp_listener = match TcpListener::bind(address).await {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(CreateServerError::BindError);
      }
    };

    Ok(Self { tcp_listener })
  }

  pub async fn accept_connection(&mut self) {
    let connection = self.tcp_listener.accept().await.unwrap();
  }
}

// GET
// CONNECT
fn begins_with_http_get_method(slice: &[u8]) -> bool {
  todo!()
}
fn begins_with_http_connect_method(slice: &[u8]) -> bool {
  todo!()
}
fn begins_with_magic_sequence(slice: &[u8]) -> bool {
  todo!()
}

pub async fn do_handshake(connection: &mut TcpStream) {
  let mut buffer: [u8; 16] = [0; 16];
  
  loop {
    let bytes_read = connection.read(&mut buffer).await.unwrap();
    if begins_with_http_get_method(&buffer) {

    } else if begins_with_http_connect_method(&buffer)  {

    } else if begins_with_magic_sequence(&buffer) {

    } else {

    }
  }
}

fn continue_http_1_get_handshake() {

}

fn continue_http_1_connect_handshake() {

}
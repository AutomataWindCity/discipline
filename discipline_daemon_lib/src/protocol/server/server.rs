use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, Semaphore};
use crate::x::{Daemon, TextualError, protocol::*};

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

struct ServerSharedData {
  tcp_listener: TcpListener,
  status: ServerStatus,
  semaphore: Arc<Semaphore>,
}

#[derive(Debug)]
pub enum CreateServerError {
  BindError(std::io::Error),
}

#[derive(Debug)]
pub enum AcceptConnectionError {
  TcpListenerAccept(std::io::Error)
}

impl ServerSharedData {
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
  server: Arc<Mutex<ServerSharedData>>,
}

impl Server {
  pub async fn new(port: u16) -> Result<Self, TextualError> {
    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    
    let tcp_listener = TcpListener::bind(&address)
      .await
      .map_err(|error| {
        TextualError::new("Creating discipline api server")
          .with_message(format!("An io error occured while binding a TCP listener to this address {}", address))
          .with_attachement_display("Io error", error)
      })?;

    Ok(Self {
      server: Arc::new(Mutex::new(ServerSharedData { 
        tcp_listener, 
        status: ServerStatus::Stopped, 
        semaphore: Arc::new(Semaphore::const_new(6))
      }))
    })
  }

  pub async fn start_auto_serving(&self, daemon: Arc<Daemon>) -> Result<(), ()> {
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

      connection.auto_serve(Arc::clone(&daemon), permit);
    }
  }

  pub async fn stop_auto_serving(self) {
    // TODO: It will take way too long to aquire a lock due to
    // the "server.accept_connection().await" in "start_auto_serving".
    // This is not a problem now because "stop_auto_serving" is never
    // called anywhere yet. 
    self.server.lock().await.status = ServerStatus::Stopped;
  }
}


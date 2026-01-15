use serde::{Serialize, Deserialize};
use crate::operating_system::{UserName, UserNameRef};
use crate::x::protocol::x::{SendErrorCode, RecvErrorCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct EstablishConnection {
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct EstablishConnectionRef<'a> {
  pub password: &'a str,
}

pub enum EstablishConnectionError {
  DatagramConnect(std::io::Error),
  SendEstablishConnectionMessage(SendErrorCode),
  RecvEstablishConnectionMessage(RecvErrorCode),
  ServerBusy,
  IncorrectPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EstablishConnectionReply {
  ServerBusy,
  IncorrectPassword,
  ConnectionEstablished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSessionOpenedNotification {
  pub user_name: UserName,
}

#[derive(Debug, Serialize)]
pub struct UserSessionOpenedNotificationRef<'a> {
  pub user_name: UserNameRef<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSessionClosedNotification {
  pub user_name: UserName,
}

#[derive(Debug, Serialize)]
pub struct UserSessionClosedNotificationRef<'a> {
  pub user_name: UserNameRef<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsUserSessionOpenPermitted {
  pub user_name: UserName,
}

#[derive(Debug, Serialize)]
pub struct IsUserSessionOpenPermittedRef<'a> {
  pub user_name: UserNameRef<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsUserSessionOpenPermittedReply {
  pub is_user_session_open_permitted: bool,
}

pub enum IsUserSessionOpenPermittedError {
  SendMessage(SendErrorCode),
  RecvReply(RecvErrorCode),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
  UserSessionOpenedNotification(UserSessionOpenedNotification),
  UserSessionClosedNotification(UserSessionClosedNotification),
  IsUserSessionOpenPermitted(IsUserSessionOpenPermitted),
}

#[derive(Debug, Serialize)]
pub enum ClientMessageRef<'a> {
  UserSessionOpenedNotification(UserSessionOpenedNotificationRef<'a>),
  UserSessionClosedNotification(UserSessionClosedNotificationRef<'a>),
  IsUserSessionOpenPermitted(IsUserSessionOpenPermittedRef<'a>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
  IsUserSessionOpenPermittedReply(IsUserSessionOpenPermittedReply),
}

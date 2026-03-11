use serde::{Serialize, Deserialize};
use super::{UserName, UserNameRef, AuthenticationToken};

#[derive(Debug, Serialize, Deserialize)]
pub struct EstablishConnection {
  pub authentication_token: AuthenticationToken,
}

#[derive(Debug, Serialize)]
pub struct EstablishConnectionRef<'a> {
  pub authentication_token: &'a AuthenticationToken,
}

pub enum EstablishConnectionError {
  ServerBusy,
  UnrecognizedAuthenticationToken,
  Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EstablishConnectionReply {
  ServerBusy,
  UnrecognizedAuthenticationToken,
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
pub struct IsUserSessionOpenBlocked {
  pub user_name: UserName,
}

#[derive(Debug, Serialize)]
pub struct IsUserSessionOpenBlockedRef<'a> {
  pub user_name: UserNameRef<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsUserSessionOpenBlockedReply {
  pub is_user_session_open_blocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
  UserSessionOpenedNotification(UserSessionOpenedNotification),
  UserSessionClosedNotification(UserSessionClosedNotification),
  IsUserSessionOpenBlocked(IsUserSessionOpenBlocked),
}

#[derive(Debug, Serialize)]
pub enum ClientMessageRef<'a> {
  UserSessionOpenedNotification(UserSessionOpenedNotificationRef<'a>),
  UserSessionClosedNotification(UserSessionClosedNotificationRef<'a>),
  IsUserSessionOpenBlocked(IsUserSessionOpenBlockedRef<'a>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
  IsUserSessionOpenBlockedReply(IsUserSessionOpenBlockedReply),
}

mod x;
mod n;
use std::ffi::CString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId {
  inner: libc::uid_t,
}

impl UserId {
  pub fn new(inner: libc::uid_t) -> Self {
    Self { 
      inner
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName {
  inner: CString
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PerUserInfo {
  user_name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrossUserInfo {

}

pub fn get_user_name_given_user_() {

}
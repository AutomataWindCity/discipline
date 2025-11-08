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

pub struct UserName {
  inner: CString
}

pub struct UserInfo {
  user_name: UserName,
}
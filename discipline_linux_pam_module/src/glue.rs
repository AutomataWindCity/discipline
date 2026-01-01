use std::{fmt::Debug, ptr};
use std::ffi::{CStr, CString};
use libc::{c_char, c_int, c_void};
use discipline_daemon::operating_system::UserName;
use crate::*;

enum GetModuleDataError {
  PamErrorWhileGettingData(i32),
  PamErrorWhileSettingData(i32),
  ErrorWhileCreatingInitialModuleData(CreateModuleError),
}

unsafe extern "C" fn cleanup(
  _pam_handle: *mut pam_handle_t,
  data: *mut c_void,
  _error_status: c_int,
) {
  let _ = unsafe { Box::from_raw(data) };
}

unsafe fn get_module_data(pamh: *mut pam_handle_t) -> Result<*const Module, GetModuleDataError> {
  let mut data: *const c_void = ptr::null();

  let status_code = unsafe {
    pam_get_data(
      pamh, 
      discipline_pam_module_data_name().as_ptr(), 
      (&mut data) as *mut *const c_void,
    )
  };

  if status_code != PAM_SUCCESS {
    return Err(GetModuleDataError::PamErrorWhileGettingData(status_code));
  }

  if !data.is_null() {
    return Ok(data as *const Module);
  }

  let data = Module::create()
    .map_err(|error| {
      GetModuleDataError::ErrorWhileCreatingInitialModuleData(error)
    })?;

  let data = Box::new(data);
  let data = Box::into_raw(data) as *mut Module;

  let status_code = unsafe {
    pam_set_data(
      pamh, 
      discipline_pam_module_data_name().as_ptr(), 
      data as *mut c_void,
      Some(cleanup),
    )
  };

  if status_code != PAM_SUCCESS {
    // free the memory.
    let _ = unsafe { Box::from_raw(data) };
    return Err(GetModuleDataError::PamErrorWhileSettingData(status_code));
  }

  Ok(data)
}

unsafe fn get_user_name(pamh: *mut pam_handle_t) -> Result<UserName, ()> {
  let mut user: *const c_char = ptr::null();

  let status_code = unsafe { 
    pam_get_user(
      pamh, 
      (&mut user) as *mut *const c_char, 
      ptr::null()
    )
  };

  if status_code != PAM_SUCCESS {
    return Err(());
  }

  Ok(UserName::new(unsafe { 
    CStr::from_ptr(user)
    // TODO: Avoid this cloning
    .to_owned()
  }))
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn pam_sm_authenticate(
//   pam_handle: *mut pam_sys::pam_handle_t,
//   flags: c_int,
//   argc: c_int,
//   argv: *mut *const c_char,
// ) -> c_int {
//   pam_sys::PAM_IGNORE
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn pam_sm_setcred(
//   pam_handle: *mut pam_sys::pam_handle_t,
//   flags: c_int,
//   argc: c_int,
//   argv: *mut *const c_char,
// ) -> c_int {
//   pam_sys::PAM_IGNORE
// }

unsafe fn set_login_denial_phrase(
  pamh: *mut pam::pam_handle_t,
  phrase: CString,
) -> Result<(), CString> {
  // PAM will free this when it's no longer needed.
  let phrase = phrase.as_ptr() as *const c_void;

  let status_code = unsafe { 
    pam::pam_set_item(pamh, pam::PAM_TEXT_INFO, phrase)
  };

  if status_code != pam::PAM_SUCCESS {
    return Err(unsafe {
      CString::from_raw(phrase as *mut i8)
    });
  }

  Ok(())
}

// unsafe fn pam_set_item(
//   pamh: *mut pam::pam_handle_t, 

//   PAM_TEXT_INFO, 
//                     "Your account is locked. Contact administrator.") {

//                     }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pam_sm_acct_mgmt(
  pamh: *mut pam_handle_t,
  _flags: c_int,
  _argc: c_int,
  _argv: *mut *const c_char,
) -> c_int {
  let data = unsafe { get_module_data(pamh) };

  let Ok(data) = data else {
    return pam::PAM_SUCCESS;
  };

  let Ok(user_name) = (unsafe { get_user_name(pamh) }) else {
    return pam::PAM_SUCCESS;
  };

  let is_login_blocked = unsafe { &*data }.is_login_blocked(&user_name);

  if is_login_blocked {
    return pam::PAM_PERM_DENIED;
  }

  pam::PAM_SUCCESS
}

pub unsafe extern "C" fn pam_sm_open_session(
  pamh: *mut pam::pam_handle_t,
  _flags: c_int,
  _argc: c_int,
  _argv: *mut *const c_char,
) -> c_int {
  let data = unsafe { get_module_data(pamh) };

  let Ok(data) = data else {
    return pam::PAM_SUCCESS;
  };

  let Ok(user_name) = (unsafe { get_user_name(pamh) }) else {
    return pam::PAM_SUCCESS;
  };

  unsafe { &*data }.on_session_opened(&user_name);

  pam::PAM_SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pam_sm_close_session(
  pamh: *mut pam_handle_t,
  _flags: c_int,
  _argc: c_int,
  _argv: *mut *const c_char,
) -> c_int {
  let data = unsafe { get_module_data(pamh) };

  let Ok(data) = data else {
    return PAM_SUCCESS;
  };

  let Ok(user_name) = (unsafe { get_user_name(pamh) }) else {
    return PAM_SUCCESS;
  };

  unsafe { &*data }.on_session_closed(&user_name);

  PAM_SUCCESS
}
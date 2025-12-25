use std::{ffi::CStr, path::Path};

const DISCIPLINE_PAM_MODULE_DATA_NAME: &str = concat!(env!("DISCIPLINE_PAM_MODULE_DATA_NAME"), "\0");
const DISCIPLINE_INSTALLATION_DIRECTORY: &str = env!("DISCIPLINE_INSTALLATION_DIRECTORY");

pub(crate) const fn discipline_pam_module_data_name() -> &'static CStr {
  unsafe {
    CStr::from_bytes_with_nul_unchecked(DISCIPLINE_PAM_MODULE_DATA_NAME.as_bytes())
  }
}

pub(crate) fn discipline_installation_directory() -> &'static Path {
  Path::new(DISCIPLINE_INSTALLATION_DIRECTORY)
}
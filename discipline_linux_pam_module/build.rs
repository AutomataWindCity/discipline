use std::path::PathBuf;

fn main() {
  println!("cargo:rustc-link-lib=pam");

  let pam_headers =
    "/workspaces/discipline/dependencies/linux-pam/libpam/include/security/pam_modules.h";

  // let pam_headers = PathBuf::new()
  //   .join("workspaces")
  //   .join("discipline")
  //   .join("dependencies")
  //   .join("linux-pam")
  //   .join("libpam")
  //   .join("include")
  //   .join("security")
  //   .join("pam_modules.h");

  let bindings = bindgen::Builder::default()
    // .header(pam_headers.to_string_lossy().to_string())
    .header(pam_headers)
    .allowlist_type("pam_.*")
    .allowlist_function("pam_.*")
    .allowlist_var("PAM_.*")
    .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
    .size_t_is_usize(true)
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from("/workspaces/discipline/discipline_linux_pam_module/src/pam_sys.rs");

  bindings
    .write_to_file(out_path)
    .expect("Couldn't write bindings!");
}

// use discipline_daemon::x::{Duration, TextualError};

// fn create_constants_file() -> Result<String, TextualError> {
//   use std::env;
//   use std::fs;
//   use std::path::Path;

//   const DISCIPLINE_INSTALLATION_DIRECTORY: &'static str = "DISCIPLINE_INSTALLATION_DIRECTORY";
//   const DISCIPLINE_PAM_MODULE_DATA_NAME: &'static str = "DISCIPLINE_PAM_MODULE_DATA_NAME";
//   const DISCIPLINE_PAM_CALL_TIMEOUT: &'static str = "DISCIPLINE_PAM_CALL_TIMEOUT";
//   const DISCIPLINE_PAM_LOGIN_BLOCKED_MESSAGE: &'static str = "DISCIPLINE_PAM_LOGIN_BLOCKED_MESSAGE";

//   let Some(discipline_installation_director) = env::var_os(DISCIPLINE_INSTALLATION_DIRECTORY) else {
//     return Err(
//       TextualError::new("Creating Rust module source code containing Discipline Linux Pam Module's compile time configuration as constants. The configuration is read from environment variables.")
//         .with_message(format!("Required environment variable {DISCIPLINE_INSTALLATION_DIRECTORY} was not set"))
//     )
//   };
//   let Some(discipline_pam_module_data_name) = env::var_os(DISCIPLINE_PAM_MODULE_DATA_NAME) else {
//     return Err(
//       TextualError::new("Creating Rust module source code containing Discipline Linux Pam Module's compile time configuration as constants. The configuration is read from environment variables.")
//         .with_message(format!("Required environment variable {DISCIPLINE_PAM_MODULE_DATA_NAME} was not set"))
//     )
//   };
//   let Some(discipline_pam_call_timeout) = env::var_os(DISCIPLINE_PAM_CALL_TIMEOUT) else {
//     return Err(
//       TextualError::new("Creating Rust module source code containing Discipline Linux Pam Module's compile time configuration as constants. The configuration is read from environment variables.")
//         .with_message(format!("Required environment variable {DISCIPLINE_PAM_CALL_TIMEOUT} was not set"))
//     )
//   };
//   let Some(discipline_pam_login_blocked_message) = env::var_os(DISCIPLINE_PAM_LOGIN_BLOCKED_MESSAGE) else {
//     return Err(
//       TextualError::new("Creating Rust module source code containing Discipline Linux Pam Module's compile time configuration as constants. The configuration is read from environment variables.")
//         .with_message(format!("Required environment variable {DISCIPLINE_PAM_LOGIN_BLOCKED_MESSAGE} was not set"))
//     )
//   };

//   let discipline_installation_director = PathBuf::from(discipline_installation_director);

//   let discipline_pam_module_data_name = CString::new(discipline_pam_module_data_name.into_vec());

//   let discipline_pam_call_timeout = discipline_pam_call_timeout
//     .into_string()
//     .map_err(|error| {
//       TextualError::new("Creating Rust module source code containing Discipline Linux Pam Module's compile time configuration as constants. The configuration is read from environment variables.")
//         .with_message(format!("Required environment variable {DISCIPLINE_PAM_CALL_TIMEOUT} is not a valid unicode string containing an integer"))
//     })?;
  
//   let discipline_pam_call
//   let timeout = env::var("TIMEOUT")
//     .expect("TIMEOUT must be set")
//     .parse::<u64>()
//     .expect("TIMEOUT must be a valid number");

//   // Generate Rust code
//   let output = format!(
//     r#"
//     use std::path::PathBuf;
//     use std::time::Duration;
    
//     pub const DISCIPLINE_INSTALLATION_DIRECTORY: PathBuf = 
//         PathBuf::from(r"{}");
//     pub const TIMEOUT: Duration = Duration::from_secs({});
//     pub const USER_ACCOUNT_BLOCKED_MESSAGE: &str = r"{}";
//     pub const PAM_MODULE_DATA_NAME: &str = r"{}";
//     "#,
//     dir,
//     timeout,
//     env::var("USER_ACCOUNT_BLOCKED_MESSAGE").unwrap(),
//     env::var("PAM_MODULE_DATA_NAME").unwrap(),
//   );

//   // Write to output file
//   let out_dir = env::var("OUT_DIR").unwrap();
//   let dest_path = Path::new(&out_dir).join("config.rs");
//   fs::write(&dest_path, output).unwrap();

//   println!("cargo:rerun-if-env-changed=DISCIPLINE_INSTALLATION_DIRECTORY");
//   println!("cargo:rerun-if-env-changed=TIMEOUT");
//   println!("cargo:rerun-if-env-changed=USER_ACCOUNT_BLOCKED_MESSAGE");
//   println!("cargo:rerun-if-env-changed=PAM_MODULE_DATA_NAME");
// }

// fn parse_duration(environment_variable: &OsString) -> Result<Duration, TextualError> {
//   let string = environment_variable
//     .into_string()
//     .map_err(|error| {
//       TextualError::new("Parsing an environment variable as an integer representing a millisecond-based duration")
//         .with_message("Environment variable is not valid unicode")
//         .with_attachement_display("Environment variable", environment_variable)
//         .with_attachement_display("")
//     })?;
// }

// pub enum OsStringToU64Error {
//   Malformed,
//   Overflow,
// }

// fn osstr_to_u64(os_str: &OsStr) -> Result<u64, OsStringToU64Error> {
//   let bytes = os_str.as_bytes();
//   let mut number: u64 = 0;
  
//   for &byte in bytes {
//     if !byte.is_ascii_digit() {
//       return Err(OsStringToU64Error::Malformed);
//     }

//     number = match number
//       .checked_mul(10)?
//       .checked_add((byte - b'0') as u64)?;
//   }

//   Some(number)
//   }
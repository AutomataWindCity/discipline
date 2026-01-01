use std::{path::PathBuf, process::Command};
use discipline_daemon::TextualError;

static LINUX_PAM_REPO_URL: &'static str = "https://github.com/linux-pam/linux-pam.git";

fn download_linux_pam(directory: PathBuf) -> Result<(), TextualError> {
  match directory.try_exists() {
    Ok(true) => {
      return Ok(());
    }
    Ok(false) => {
      // noop
    }
    Err(error) => {
      return Err(
        TextualError::new("Downloading Linux-PAM repo")
          .with_message("An io error occured while checking whether Linux-PAM is already downloaded by checking whether its download directory exists.")
          .with_attachement_display("Download directory", directory.display())
          .with_attachement_display("Io error", error)
      );
    }
  }

  let output = Command::new("git")
    .arg("clone")
    .arg(LINUX_PAM_REPO_URL)
    .arg(&directory)
    .output();

  let output = match output {
    Ok(value) => {
      value
    }
    Err(error) => {
      return Err(
        TextualError::new("Downloading Linux-PAM repo")
          .with_message("An io error occured while running 'git clone'")
          .with_attachement_display("Repo url", LINUX_PAM_REPO_URL)
          .with_attachement_display("Download directory", directory.display())
          .with_attachement_display("Io error", error)
      );
    }
  };

  if output.status.success() {
    return Ok(());
  }

  Err(match String::from_utf8(output.stderr) {
    Ok(stderr) => {
      TextualError::new("Downloading Linux-PAM repo")
        .with_message("Command 'git clone' exited with a non-success code")
        .with_attachement_display("Repo url", LINUX_PAM_REPO_URL)
        .with_attachement_display("Download directory", directory.display())
        .with_attachement_display("Stderr", stderr)
    }
    Err(error) => {
      TextualError::new("Downloading Linux-PAM repo")
        .with_message("Command 'git clone' exited with a non-success code and stderr couldn't be converted to String")
        .with_attachement_display("Repo url", LINUX_PAM_REPO_URL)
        .with_attachement_display("Download directory", directory.display())
        .with_attachement_debug("Stderr as bytes", error.as_bytes())
        .with_attachement_display("Create String from utf8 error", error)
    }
  })
}

fn main() {
  // println!("cargo:rustc-link-lib=pam");

  let crate_directory = PathBuf::from("/workspaces/discipline/dependencies/pam");

  // let crate_directory = match std::env::var_os("CRATE_DIR") {
  //   Some(value) => {
  //     PathBuf::from(value)
  //   }
  //   None => {
  //     panic!("CRATE_DIR is not defined");
  //   }
  // };

  let bindings_file = crate_directory
    .join("src")
    .join("bindings.rs");

  let linux_pam_directory = crate_directory
    .join("build")
    .join("linux_pam");

  let linux_pam_headers_file = linux_pam_directory
    .join("libpam")
    .join("include")
    .join("security")
    .join("pam_modules.h");

  download_linux_pam(linux_pam_directory).unwrap();

  let bindings = bindgen::Builder::default()
    // .header(pam_headers.to_string_lossy().to_string())
    .header(linux_pam_headers_file.to_string_lossy())
    .allowlist_type("pam_.*")
    .allowlist_function("pam_.*")
    .allowlist_var("PAM_.*")
    .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
    .size_t_is_usize(true)
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(bindings_file)
    .expect("Couldn't write bindings!");
}
use std::{path::PathBuf, process::Command};

pub struct Installer {
  jdk_url: String,
  jdk_temp_directory_path: PathBuf,
  jdk_temp_file_path: PathBuf,
  jdk_directory_path: PathBuf,
  jdk_file_path: PathBuf,
}

impl Installer {
  pub fn install_curl(&self) {
    let it = Command::new("sudo")
      .arg("apt")
      .arg("update")
      .output();

    let it = match it {
      Ok(it) => {
        it
      }
      Err(error) => {
        return;
      }
    };

    let it = Command::new("sudo")
      .arg("apt")
      .arg("install")
      .arg("curl")
      .output();
  }
}



fn install_jdk(
  url: &str,
  
) {
  const URL: &str = "https://download.oracle.com/java/25/latest/jdk-25_linux-x64_bin.deb";
}

fn main() {
  println!("Hello, world!");
}

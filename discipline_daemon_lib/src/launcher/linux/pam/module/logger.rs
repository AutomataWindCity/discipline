use std::fmt::Debug;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};

type SysLogger = syslog::Logger<syslog::LoggerBackend, syslog::Formatter3164>;

struct SystemLog {
  log: Option<SysLogger>,
}

impl SystemLog {
  fn new() -> Self {
    Self {
      log: None,
    }
  }

  fn write(&mut self, message: &str) -> Result<(), syslog::Error> {
    if let Some(log) = &mut self.log {
      return log.err(message);
    }

    let mut log = match open_syslog() {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(error);
      }
    };

    let result = log.err(message);
    self.log = Some(log);
    result
  }
}

struct CustomLog {
  path: PathBuf,
  log: Option<File>,
}

impl CustomLog {
  fn new(path: PathBuf) -> Self {
    Self {
      log: None,
      path,
    }
  }

  fn write(&mut self, message: &str) -> Result<(), std::io::Error> {
    if let Some(log) = &mut self.log {
      return log.write_all(message.as_bytes());
    }

    let mut log = match open_file(&self.path) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(error);
      }
    };

    let result = log.write_all(message.as_bytes());
    self.log = Some(log);
    result
  }
}

struct Log {
  system: SystemLog,
  custom: CustomLog,
}

fn open_file(path: &Path) -> Result<File, std::io::Error> {
  OpenOptions::new()
    .create(true)
    .append(true)
    .write(true)
    .open(path)
}

fn open_syslog() -> Result<syslog::Logger<syslog::LoggerBackend, syslog::Formatter3164>, syslog::Error> {
  let formatter = syslog::Formatter3164 {
    facility: syslog::Facility::LOG_AUTHPRIV,
    hostname: None,
    process: "Discipline Linux-PAM Module".into(),
    pid: std::process::id(),
  };

  syslog::unix(formatter)
}

pub struct Logger {
  system: SystemLog,
  custom: CustomLog,
}

impl Logger {
  pub fn create(custom_log_file_path: PathBuf) -> Self {
    Self {
      system: SystemLog::new(),
      custom: CustomLog::new(custom_log_file_path),
    }
  }

  pub fn write_str(&mut self, message: &str) {
    let Err(system_log_error) = self.system.write(message) else {
      return;
    };

    let Err(custom_log_error) = self.custom.write(message) else {
      return;
    };

    // TODO: Log "system_log_error" and "custom_log_error" somehow.
    // TODO: Add a compile-time option specifying whether we panic or not when we cannot log.
  }

  pub fn write_debugable(&mut self, message: impl Debug) {
    
  }

  pub fn write_displayable(&mut self, message: impl Debug) {
    
  }
}
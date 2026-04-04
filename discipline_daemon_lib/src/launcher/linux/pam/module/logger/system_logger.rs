use std::time::Instant;
use syslog::{Error, Formatter3164, Logger, LoggerBackend};
use crate::x::Duration;

type ActualLogger = Logger<LoggerBackend, Formatter3164>;

fn open() -> Result<ActualLogger, Error> {
  let formatter = Formatter3164 {
    facility: syslog::Facility::LOG_AUTHPRIV,
    hostname: None,
    process: "Discipline Linux-PAM Module".into(),
    pid: std::process::id(),
  };

  syslog::unix(formatter)
}

pub struct SystemLogger {
  log: Option<ActualLogger>,
  reopen_interval: Duration,
  maximum_reopens_per_interval: u8,
  reopen_window_started_at: Instant,
  reopen_count_within_window: u8,
}

impl SystemLogger {
  fn new() -> Self {
    Self {
      log: None,
      reopen_interval: Duration::from_milliseconds(1_000),
      maximum_reopens_per_interval: 4,
      reopen_window_started_at: Instant::now(),
      reopen_count_within_window: 0,
    }
  }

  fn write(&mut self, message: &str) -> Result<(), Error> {
    if let Some(log) = &mut self.log {
      if let Ok(()) = log.err(message) {
        return Ok(());
      }

      self.log = None;
    }

    // let now = Instant::now();
    // if now.duration_since(self.reopen_window_started_at) >= self.reopen_interval.to_std_duration() {
    //   self.reopen_window_started_at = now;
    //   self.reopen_count_within_window = 0;
    // }

    // if self.reopen_count_within_window >= self.maximum_reopens_per_interval {
    //   return Err(Error::Io(std::io::Error::other("syslog reopen rate limit exceeded")));
    // }

    // self.reopen_count_within_window = self.reopen_count_within_window.saturating_add(1);

    let mut log = open()?;

    let result = log.err(message);
    self.log = Some(log);
    result
  }
}

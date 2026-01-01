pub struct Logger {
  inner: syslog::Logger<syslog::LoggerBackend, syslog::Formatter3164>,
}

pub type CreateError = syslog::Error;

impl Logger {
  pub fn create() -> Result<Self, syslog::Error> {
    let formatter = syslog::Formatter3164 {
      facility: syslog::Facility::LOG_AUTHPRIV,
      hostname: None,
      process: "Discipline Linux-PAM Module".into(),
      pid: std::process::id(),
    };

    syslog::unix(formatter).map(|inner| {
      Self { inner }
    })
  }
}
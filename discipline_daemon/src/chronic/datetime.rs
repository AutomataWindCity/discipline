use crate::x::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
  inner: chrono::DateTime<chrono::Utc>,
}

impl DateTime {
  pub fn now() -> DateTime {
    Self {
      inner: chrono::Utc::now(),
    }
  }

  pub fn timestamp_in_milliseonds(self) -> i64 {
    self.inner.timestamp_millis()
  }

  pub fn till_or_zero(self, later: DateTime) -> Duration {
    match later
      .timestamp_in_milliseonds()
      .checked_sub(self.timestamp_in_milliseonds())
    {
      None => {
        Duration::zero()
      }
      Some(milliseconds) => {
        Duration::from_milliseconds(milliseconds.try_into().unwrap())
      }
    }
  }
}
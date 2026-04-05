use serde::{Deserialize, Serialize};
use crate::x::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownState {
  Pending,
  Running,
  Finished,
}

impl CountdownState {
  pub fn is_pending(self) -> bool {
    matches!(self, CountdownState::Pending)
  }

  pub fn is_running(self) -> bool {
    matches!(self, CountdownState::Running)
  }

  pub fn is_finished(self) -> bool {
    matches!(self, CountdownState::Finished)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  pub from: Instant,
  pub duration: Duration,
}

impl Countdown {
  pub fn create(from: Instant, duration: Duration) -> Self {
    Self {
      from,
      duration,
    }
  }

  pub fn construct(from: Instant, duration: Duration) -> Countdown {
    Self { 
      from, 
      duration,
    }
  }

  pub fn get_from(&self) -> Instant {
    self.from
  }

  pub fn get_till(&self) -> Instant {
    self.from.saturating_add(self.duration)
  }

  pub fn get_total_duration(&self) -> Duration {
    self.duration
  }

  pub fn set_total_duration(&mut self, new_value: Duration) {
    self.duration = new_value;
  }

  pub fn get_duration_till_start_or_zero(&self, now: Instant) -> Duration {
    now.till_or_zero(self.from)
  }

  pub fn get_duration_since_start_or_zero(&self, now: Instant) -> Duration {
    now.since_or_zero(self.from)
  }

  pub fn get_elapsed_time_or_zero(&self, now: Instant) -> Duration {
    self.get_duration_since_start_or_zero(now).min(self.duration)
  }

  pub fn get_remaining_time_or_zero(&self, now: Instant) -> Duration {
    self.get_total_duration().saturating_sub(self.get_elapsed_time_or_zero(now))
  }

  pub fn get_time_till_finish_or_zero(&self, now: Instant) -> Duration {
    now.till_or_zero(self.get_till())
  }

  pub fn get_state(&self, now: Instant) -> CountdownState {
    if now.is_eariler_than(self.from) {
      return CountdownState::Pending;
    } 
    
    let elapsed_time = self.get_elapsed_time_or_zero(now);
    if elapsed_time.is_shorter_than_or_equal_to(self.duration) {
      return CountdownState::Running;
    }
    
    CountdownState::Finished
  }

  // todo: delete
  pub fn get_remaining_duration_or_zero(&self, now: Instant) -> Duration {
    match self.get_state(now) {
      CountdownState::Pending => {
        self.duration
      }
      CountdownState::Running => {
        self.duration.saturating_sub(self.from.till_or_zero(now))
      }
      CountdownState::Finished => {
        Duration::zero()
      }
    }
  }

  pub fn is_pending(&self, now: Instant) -> bool {
    self.get_state(now).is_pending()
  }

  pub fn is_running(&self, now: Instant) -> bool {
    self.get_state(now).is_running()
  }

  pub fn is_finished(&self, now: Instant) -> bool {
    self.get_state(now).is_finished()
  }
}
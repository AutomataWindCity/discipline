use crate::x::{DateTime, Duration};

pub struct MonotonicClock {
  total_elapsed_duration: Duration,
  previous_synchronization_realtime: Instant,
  previous_synchronization_boottime: Instant,
  maximum_synchronization_interval: Duration,
}

impl MonotonicClock {
  pub fn create(
    realtime: Instant,
    boottime: Instant,
    maximum_synchronization_interval: Duration,
  ) -> Self {
    Self {
      total_elapsed_duration: Duration::zero(),
      previous_synchronization_realtime: realtime,
      previous_synchronization_boottime: boottime,
      maximum_synchronization_interval,
    } 
  }
  
  pub fn construct(
    realtime: Instant,
    boottime: Instant,
    total_elapsed_duration: Duration,
    maximum_synchronization_interval: Duration,
    previous_synchronization_realtime: Instant,
    previous_synchronization_boottime: Instant,
  ) -> Self {
    let mut clock = Self {
      total_elapsed_duration,
      maximum_synchronization_interval,
      previous_synchronization_realtime,
      previous_synchronization_boottime,
    };

    clock.synchronize_with_realtime(realtime);

    clock
  }

  pub fn now(&self) -> Instant {
    Instant(self.total_elapsed_duration)
  }

  pub fn total_elapsed_duration(&self) -> Duration {
    self.total_elapsed_duration
  }

  pub fn previous_synchronization_time(&self) -> Option<DateTime> {
    self.previous_synchronization_time
  }

  pub fn synchronization_interval(&self) -> Duration {
    self.maximum_synchronization_interval
  }
  
  fn synchronize_with_realtime(
    &mut self,
    realtime: Instant,
  ) {

    // let Some(realtime) = get_time_from_boottime_clock() else {
    //   return;
    // };

    let realtime_since_prev_sync = self
      .previous_synchronization_realtime
      .till_or_zero(realtime);

    if realtime_since_prev_sync.is_zero() {
      return;
    }

    self.total_elapsed_duration = self
      .total_elapsed_duration
      .saturating_add(realtime_since_prev_sync)
      .min(self.maximum_synchronization_interval);
    
    self.previous_synchronization_realtime = realtime;
  }

  pub fn synchronize(
    &mut self,
    realtime: Instant,
    boottime: Instant,
  ) {
    self.previous_synchronization_realtime = realtime;

    let boottime_since_prev_sync = self 
      .previous_synchronization_boottime
      .till_or_zero(boottime);

    if boottime_since_prev_sync.is_zero() {
      return;
    }

    self.total_elapsed_duration = self
      .total_elapsed_duration
      .saturating_add(boottime_since_prev_sync)
      .min(self.maximum_synchronization_interval);

    self.previous_synchronization_boottime = boottime;
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(Duration);

impl Instant {
  pub const MAX: Instant = Instant(Duration::MAX);

  pub fn from_timestamp(timestamp: u64) -> Self {
    Self(Duration::from_milliseconds(timestamp))
  }

  pub fn from_elapsed_time(elapsed_time: Duration) -> Self {
    Self(elapsed_time)
  }
  
  pub fn is_eariler_than(self, other: Instant) -> bool {
    self.0.is_shorter_than(other.0)
  }
  
  pub fn is_eariler_than_at(self, other: Instant) -> bool {
    self.0.is_shorter_than_or_equal_to(other.0)
  }

  pub fn is_at(self, other: Instant) -> bool {
    self.0.is_equal_to(other.0)
  }

  pub fn is_later_than_or_at(self, other: Instant) -> bool {
    self.0.is_longer_than_or_equal_to(other.0)
  }

  pub fn is_later_than(self, other: Instant) -> bool {
    self.0.is_longer_than(other.0)
  }

  pub fn till_or_zero(self, later: Instant) -> Duration {
    later.0.minus_or_zero(self.0)
  }

  pub fn since_or_zero(self, eariler: Instant) -> Duration {
    eariler.0.minus_or_zero(self.0)
  }

  pub fn saturating_add(self, duration: Duration) -> Instant {
    Self(self.0.saturating_add(duration))
  }

  pub fn saturating_sub(self, duration: Duration) -> Instant {
    Self(self.0.saturating_sub(duration))
  }
  
  pub fn as_elapsed_time(self) -> Duration {
    self.0
  }

  pub fn as_timestamp(&self) -> u64 {
    self.0.as_total_milliseconds()
  }
}

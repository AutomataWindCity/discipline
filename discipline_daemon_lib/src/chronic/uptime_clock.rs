use crate::x::{Duration, Instant};

const DAY: Duration = Duration::day();
const WEEK: Duration = Duration::week();

pub fn get_monotonic_time() -> Option<Instant> {
  let mut timespec = libc::timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
    
  // CLOCK_MONOTONIC represents a monotonic clock that never jumps backwards
  // and is unaffected by system time adjustments
  let result = unsafe {
    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut timespec)
  };
  
  if result != 0 {
    return None;
  }

  // let seconds: u64 = timespec.tv_sec.try_into().ok()?;
  // let milliseconds = seconds.checked_mul(1000)?;
  
  Some(Instant::from_timestamp(
    timespec
      .tv_sec
      .try_into()
      .ok()
      .checked_mul(1000)?
  ))
}

#[derive(Debug, Clone)]
pub struct UserUptimeClock {
  pub is_running: bool,
  pub day_start: Instant,
  pub day_uptime: Duration,
  pub week_start: Instant,
  pub week_uptime: Duration,
  pub previous_synchronization_time: Instant,
  pub maximum_synchronization_interval: Duration,
}

impl UserUptimeClock {
  fn create(
    today: Instant,
    maximum_synchronization_interval: Duration,
  ) -> Self {
    UserUptimeClock {
      is_running: false,
      day_start: today,
      day_uptime: Duration::zero(),
      week_start: today,
      week_uptime: Duration::zero(),
      previous_synchronization_time: today,
      maximum_synchronization_interval,
    }
  }

  pub fn construct(
    is_running: bool,
    day_start: Instant,
    day_uptime: Duration,
    week_start: Instant,
    week_uptime: Duration,
    previous_synchronization_time: Instant,
    maximum_synchronization_interval: Duration,
  ) -> Self {
    Self {
      is_running,
      day_start,
      day_uptime,
      week_start,
      week_uptime,
      previous_synchronization_time,
      maximum_synchronization_interval,
    }
  }

  pub fn synchronize(
    &mut self,
    now: Instant,
  ) {
    let mut time_since_prev_sync = self.previous_synchronization_time.till_or_zero(now);
    if time_since_prev_sync.is_zero() {
      return;
    }
    if time_since_prev_sync.is_longer_than(self.maximum_synchronization_interval) {
      time_since_prev_sync = self.maximum_synchronization_interval;
    }
    
    let time_since_day_start = self.day_start.till_or_zero(now);
    if time_since_day_start.is_shorter_than(DAY) {
      self.day_uptime = self
        .day_uptime
        .saturating_add(time_since_prev_sync)
    } else {
      self.day_start = self
        .day_start
        .plus_or_max(DAY);

      self.day_uptime = self
        .day_start
        .till_or_zero(now)
        .min(self.maximum_synchronization_interval);
    }

    let time_since_week_start = self.week_start.till_or_zero(now);
    if time_since_day_start.is_shorter_than(WEEK) {
      self.week_uptime = self
        .week_uptime
        .saturating_add(time_since_prev_sync)
    } else {
      self.week_start = self
        .week_start
        .plus_or_max(WEEK);

      self.week_uptime = self
        .week_start
        .till_or_zero(now)
        .min(self.maximum_synchronization_interval);
    }

    self.previous_synchronization_time = now;
  }
}
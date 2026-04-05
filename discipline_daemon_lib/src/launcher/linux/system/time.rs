use crate::x::{Instant, Duration};

pub fn get_time_from_realtime_clock() -> Option<Instant> {
  let mut timespec = libc::timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  
  let result = unsafe {
    libc::clock_gettime(libc::CLOCK_REALTIME, &mut timespec)
  };
  
  if result != 0 {
    return None;
  } 
  
  let timestamp: u64 = timespec
      .tv_sec
      .try_into()
      .ok()?;

  Some(Instant::from_timestamp(
    timestamp.checked_mul(Duration::MILLISECONDS_PER_SECOND)?
  ))
}

/// Get monotonic raw time (including time spent in suspend)
pub fn get_time_from_boottime_clock() -> Option<Instant> {
  let mut timespec = libc::timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  
  let result = unsafe {
    libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut timespec)
  };
  
  if result != 0 {
    return None;
  } 
  
  let timestamp: u64 = timespec
      .tv_sec
      .try_into()
      .ok()?;

  Some(Instant::from_timestamp(
    timestamp.checked_mul(Duration::MILLISECONDS_PER_SECOND)?
  ))
}

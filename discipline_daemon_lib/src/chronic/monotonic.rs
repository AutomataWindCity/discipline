use std::sync::Arc;
use tokio::{sync::RwLock, time::sleep};
use crate::x::{DateTime, Duration};

pub struct MonotonicClock {
  milliseconds: u64,
  previous_synchronization_time: Option<DateTime>,
  synchronization_interval: Duration,
}

impl Default for MonotonicClock {
  fn default() -> Self {
    Self {
      milliseconds: 1,
      previous_synchronization_time: None,
      synchronization_interval: Duration::from_minutes_or_panic(5)
    }
  }
}

impl MonotonicClock {
  pub fn now(&self) -> MonotonicInstant {
    MonotonicInstant { timestamp: self.milliseconds }
  }
}

pub struct SharedMonotonicClock {
  clock: Arc<RwLock<MonotonicClock>>,
}

fn synchronization_loop_iteration(clock: &mut MonotonicClock) {
  let current_time = DateTime::now();

  let previous_synchronization_time = match clock.previous_synchronization_time {
    None => {
      clock.previous_synchronization_time = Some(current_time);
      return;
    }
    Some(time) => {
      time
    }
  };

  let interval = previous_synchronization_time
    .till_or_zero(current_time);

  // TODO: Log an error if "clock.milliseconds" reaches the maximum value for
  // "u64".
  clock.milliseconds = clock.milliseconds.saturating_add(interval.milliseconds());

  // TODO: Update the database, too.
}

impl SharedMonotonicClock {
  pub async fn start_synchronization_loop(self) {
    loop {
      let mut clock_guard = self.clock.write().await;
      let clock = &mut *clock_guard;
      let interval = clock.synchronization_interval.to_std_duration();
      
      synchronization_loop_iteration(clock);
      drop(clock_guard);
      sleep(interval).await;
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotonicInstant {
  timestamp: u64,
}

impl MonotonicInstant {
  pub const MAX: MonotonicInstant = MonotonicInstant { timestamp: u64::MAX };

  pub fn from_timestamp(timestamp: u64) -> Self {
    Self { timestamp }
  }
  
  pub fn is_eariler_than(self, other: MonotonicInstant) -> bool {
    self.timestamp < other.timestamp
  }

  pub fn is_later_than(self, other: MonotonicInstant) -> bool {
    self.timestamp > other.timestamp
  }

  pub fn is_at(self, other: MonotonicInstant) -> bool {
    self.timestamp == other.timestamp
  }

  pub fn till_or_zero(self, other: MonotonicInstant) -> Duration {
    other
      .timestamp
      .checked_sub(self.timestamp)
      .map(Duration::from_milliseconds)
      .unwrap_or_else(Duration::zero)
  }

  pub fn timestamp(&self) -> u64 {
    self.timestamp
  }
}

mod serialization {
  use serde::{Serialize, Deserialize};
  use super::{MonotonicInstant};

  impl Serialize for MonotonicInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.timestamp.serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for MonotonicInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u64::deserialize(deserializer).map(|timestamp| MonotonicInstant { timestamp })
    }
  }
}

pub mod database {
  use crate::x::database::*;
  use super::MonotonicClock;

  pub struct Schema {
    milliseconds: Key,
    previous_synchronization_time: Key,
    synchronization_interval: Key,
  }

  impl Schema {
    pub fn new(
      milliseconds: Key,
      previous_synchronization_time: Key,
      synchronization_interval: Key,
    ) -> Self {
      Self {
        milliseconds,
        previous_synchronization_time,
        synchronization_interval,
      }
    }
  }

  impl WriteCompoundValue for MonotonicClock {
    type Schema = Schema;

    fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
      destination.write_scalar_value(schema.milliseconds, &value.milliseconds);
      destination.write_scalar_value(schema.previous_synchronization_time, &value.previous_synchronization_time);
      destination.write_scalar_value(schema.synchronization_interval, &value.synchronization_interval);
    }
  }

  impl ReadCompoundValue for MonotonicClock {
    type Schema = Schema;

    fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, crate::x::TextualError> {
      Ok(MonotonicClock { 
        // TODO: Should we err if this returns 0?
        milliseconds: source.read_scalar_value(schema.milliseconds)?,
        previous_synchronization_time: source.read_scalar_value(schema.previous_synchronization_time)?,
        synchronization_interval: source.read_scalar_value(schema.synchronization_interval)?,
      })
    }
  }
}

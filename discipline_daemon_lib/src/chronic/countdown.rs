use serde::{Deserialize, Serialize};
use crate::x::{Duration, MonotonicInstant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  pub from: MonotonicInstant,
  pub duration: Duration,
}

impl Countdown {
  pub fn new(duration: Duration) -> Countdown {
    Countdown { from: MonotonicInstant::MAX, duration }
  }

  pub fn construct(from: MonotonicInstant, duration: Duration) -> Countdown {
    Countdown { from, duration }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn remaining_duration(&self, now: MonotonicInstant) -> Duration {
    self.from.till_or_zero(now).minus_or_zero(self.duration)
  }

  pub fn is_finished(&self, now: MonotonicInstant) -> bool {
    self.remaining_duration(now).is_zero()
  }

  pub fn is_running(&self, now: MonotonicInstant) -> bool {
    !self.remaining_duration(now).is_zero()
  }

  pub fn begin(&mut self, now: MonotonicInstant) {
    self.from = now;
  }

  pub fn cancel(&mut self) {
    self.from = MonotonicInstant::MAX;
  }
}

// pub mod database {
//   use crate::x::database::*;
//   use crate::x::Countdown;

//   pub struct Schema {
//     from: Key,
//     duration: Key,
//   }

//   impl Schema {
//     pub fn new(from: Key, duration: Key) -> Self {
//       Self { from, duration }
//     }
//   }

//   impl SerializableCompoundValue for Countdown {
//     type Schema = Schema;

//     fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
//       writer.write_scalar_value(schema.from, &value.from);
//       writer.write_scalar_value(schema.duration, &value.duration);
//     }
//   }
// }
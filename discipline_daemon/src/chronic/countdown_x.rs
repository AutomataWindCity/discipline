use serde::{Deserialize, Serialize};
use crate::x::{Duration, time_x};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownX {
  pub from: time_x::InstantX,
  pub duration: Duration,
}

impl CountdownX {
  pub fn new(duration: Duration) -> CountdownX {
    CountdownX { from: time_x::InstantX::MAX, duration }
  }

  pub fn construct(from: time_x::InstantX, duration: Duration) -> CountdownX {
    CountdownX { from, duration }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn remaining_duration(&self, now: time_x::InstantX) -> Duration {
    self.from.till_or_zero(now).minus_or_zero(self.duration)
  }

  pub fn is_finished(&self, now: time_x::InstantX) -> bool {
    self.remaining_duration(now).is_zero()
  }

  pub fn is_running(&self, now: time_x::InstantX) -> bool {
    !self.remaining_duration(now).is_zero()
  }

  pub fn begin(&mut self, now: time_x::InstantX) {
    self.from = now;
  }

  pub fn cancel(&mut self) {
    self.from = time_x::InstantX::MAX;
  }
}

pub mod snapshoot {
  use crate::x::{countdown_x, time_x};

  pub struct Snapshoot {
    from: time_x::InstantX,
  }

  impl Snapshoot {
    pub fn revert_changes(&self, countdown: &mut countdown_x::CountdownX) {
      countdown.from = self.from;
    }
  }
}

pub mod database {
  use crate::x::database::*;
  use crate::x::countdown_x::CountdownX;

  pub struct Schema {
    from: Key,
    duration: Key,
  }

  impl Schema {
    pub fn new(from: Key, duration: Key) -> Self {
      Self { from, duration }
    }
  }

  impl SerializableCompoundValue for CountdownX {
    type Schema = Schema;

    fn serialize(value: &Self, schema: &Self::Schema, writer: &mut impl CompoundValueWriter) {
      writer.write_scalar_value(schema.from, &value.from);
      writer.write_scalar_value(schema.duration, &value.duration);
    }
  }
}
use crate::x::Duration;

pub struct Clock {
  milliseconds: u64,
}

impl Clock {
  pub fn now(&self) -> InstantX {
    InstantX { timestamp: self.milliseconds }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstantX {
  timestamp: u64,
}

impl InstantX {
  pub const MAX: InstantX = InstantX { timestamp: u64::MAX };

  pub fn is_eariler_than(self, other: InstantX) -> bool {
    self.timestamp < other.timestamp
  }

  pub fn is_later_than(self, other: InstantX) -> bool {
    self.timestamp > other.timestamp
  }

  pub fn is_at(self, other: InstantX) -> bool {
    self.timestamp == other.timestamp
  }

  pub fn till_or_zero(self, other: InstantX) -> Duration {
    other
      .timestamp
      .checked_sub(self.timestamp)
      .map(Duration::from_milliseconds)
      .unwrap_or_else(Duration::zero)
  }
}

mod serialization {
  use serde::{Serialize, Deserialize};
  use super::{InstantX};

  impl Serialize for InstantX {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.timestamp.serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for InstantX {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u64::deserialize(deserializer).map(|timestamp| InstantX { timestamp })
    }
  }
}

mod database {
  use crate::x::database::*;
  use crate::x::time_x::InstantX;

  impl SerializableScalarValue for InstantX {
    fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
      writer.write_scalar_value(&value.timestamp);
    }
  }

  impl DeserializableScalarValue for InstantX {
    fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, crate::x::TextualError> {
      reader.read_scalar_value().map(|timestamp| InstantX { timestamp })
    }
  }
}
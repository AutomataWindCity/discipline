use crate::x::Weekday;
use std::fmt::{self, Debug};

// Invariant: the 8-th bit is always 0.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeekdaySet(u8);

impl WeekdaySet {
  const MON_BITMASK: u8 = 0b000_0001;
  const TUE_BITMASK: u8 = 0b000_0010;
  const WED_BITMASK: u8 = 0b000_0100;
  const THU_BITMASK: u8 = 0b000_1000;
  const FRI_BITMASK: u8 = 0b001_0000;
  const SAT_BITMASK: u8 = 0b010_0000;
  const SUN_BITMASK: u8 = 0b100_0000;

  const MON_ONLY_SET: Self = Self(Self::MON_BITMASK);
  const TUE_ONLY_SET: Self = Self(Self::TUE_BITMASK);
  const WED_ONLY_SET: Self = Self(Self::WED_BITMASK);
  const THU_ONLY_SET: Self = Self(Self::THU_BITMASK);
  const FRI_ONLY_SET: Self = Self(Self::FRI_BITMASK);
  const SAT_ONLY_SET: Self = Self(Self::SAT_BITMASK);
  const SUN_ONLY_SET: Self = Self(Self::SUN_BITMASK);

  const ALL: Self = Self(0b111_1111);
  const EMPTY: Self = Self(0b000_0000);

  pub fn from_bitmask(bitmask: u8) -> Self {
    // Set last bit to zero
    Self(bitmask & 0x7F)
  }

  pub fn bitmask(&self) -> u8 {
    self.0
  }

  pub fn from_weekday(weekday: Weekday) -> Self {
    match weekday {
      Weekday::Mon => Self::MON_ONLY_SET,
      Weekday::Tue => Self::TUE_ONLY_SET,
      Weekday::Wed => Self::WED_ONLY_SET,
      Weekday::Thu => Self::THU_ONLY_SET,
      Weekday::Fri => Self::FRI_ONLY_SET,
      Weekday::Sat => Self::SAT_ONLY_SET,
      Weekday::Sun => Self::SUN_ONLY_SET,
    }
  }

  pub fn to_weekday(self) -> Option<Weekday> {
    match self {
      Self::MON_ONLY_SET => Some(Weekday::Mon),
      Self::TUE_ONLY_SET => Some(Weekday::Tue),
      Self::WED_ONLY_SET => Some(Weekday::Wed),
      Self::THU_ONLY_SET => Some(Weekday::Thu),
      Self::FRI_ONLY_SET => Some(Weekday::Fri),
      Self::SAT_ONLY_SET => Some(Weekday::Sat),
      Self::SUN_ONLY_SET => Some(Weekday::Sun),
      _ => None,
    }
  }

  pub fn add(&mut self, weekday: Weekday) {
    if !self.contains(weekday) {
      self.0 |= Self::from_weekday(weekday).0;
    }
  }

  pub fn remove(&mut self, weekday: Weekday) {
    if self.contains(weekday) {
      self.0 &= !Self::from_weekday(weekday).0;
    }
  }

  pub fn contains(self, day: Weekday) -> bool {
    self.0 & Self::from_weekday(day).0 != 0
  }

  pub fn is_empty(self) -> bool {
    self == Self::EMPTY
  }
}

/// Print the underlying bitmask, padded to 7 bits.
///
/// # Example
/// ```
/// # use chrono::Self;
/// use chrono::Weekday::*;
/// assert_eq!(format!("{:?}", Self::single(Mon)), "Self(0000001)");
/// assert_eq!(format!("{:?}", Self::single(Tue)), "Self(0000010)");
/// assert_eq!(format!("{:?}", Self::ALL), "Self(1111111)");
/// ```
impl Debug for WeekdaySet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Self({:0>7b})", self.0)
  }
}

// impl fmt::Display for WeekdaySet {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//   write!(f, "WeekdaySet[")?;

//   let mut x = false;

//   if self.contains(Weekday::Mon) {
//     write!(f, "Mon, ")
//   }

//   let mut first = true;
//   for (weekday, name) in days.iter() {
//     if self.contains(*weekday) {
//       if !first {
//         write!(f, ", ")?;
//       }
//       write!(f, "{}", name)?;
//       first = false;
//     }
//   }

//   write!(f, "]")
//   }
// }

mod serialization {
  use crate::x::WeekdaySet;
  use serde::{Deserialize, Serialize};

  impl Serialize for WeekdaySet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer 
    {
      self.bitmask().serialize(serializer)
    }
  }

  impl<'a> Deserialize<'a> for WeekdaySet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'a> 
    {
      u8::deserialize(deserializer).map(WeekdaySet::from_bitmask)
    }
  }
}

use core::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Weekday {
  /// Monday.
  Mon = 0,
  /// Tuesday.
  Tue = 1,
  /// Wednesday.
  Wed = 2,
  /// Thursday.
  Thu = 3,
  /// Friday.
  Fri = 4,
  /// Saturday.
  Sat = 5,
  /// Sunday.
  Sun = 6,
}

// pub enum CreateFRomNu

impl Weekday {
  pub fn from_number_from_monday(number: u8) -> Option<Weekday> {
    match number {
      0 => Some(Weekday::Mon),
      1 => Some(Weekday::Tue),
      2 => Some(Weekday::Wed),
      3 => Some(Weekday::Thu),
      4 => Some(Weekday::Fri),
      5 => Some(Weekday::Sat),
      6 => Some(Weekday::Sun),
      _ => None,
    }
  }

  pub fn from_number_from_sunday(number: u8) -> Option<Weekday> {
    match number {
      1 => Some(Weekday::Sun),
      2 => Some(Weekday::Mon),
      3 => Some(Weekday::Tue),
      4 => Some(Weekday::Wed),
      5 => Some(Weekday::Thu),
      6 => Some(Weekday::Fri),
      7 => Some(Weekday::Sat),
      _ => None,
    }
  }

  pub unsafe fn unchecked_from_number_from_monday(number: u8) -> Weekday {
    match number {
      0 => Weekday::Mon,
      1 => Weekday::Tue,
      2 => Weekday::Wed,
      3 => Weekday::Thu,
      4 => Weekday::Fri,
      5 => Weekday::Sat,
      6 => Weekday::Sun,
        _ => panic!("Creating Weekday from number from monday: expected a number in this range 0..=6 but found {number}"),
    }
  }

  pub unsafe fn unchecked_from_number_from_sunday(number: u8) -> Weekday {
    match number {
      0 => Weekday::Sun,
      1 => Weekday::Mon,
      2 => Weekday::Tue,
      3 => Weekday::Wed,
      4 => Weekday::Thu,
      5 => Weekday::Fri,
      6 => Weekday::Sat,
      _ => panic!("Creating Weekday from number from sunday: expected a number in this range 0..=6 but found {number}"),
    }
  }

  pub const fn successor(&self) -> Weekday {
    match *self {
      Weekday::Mon => Weekday::Tue,
      Weekday::Tue => Weekday::Wed,
      Weekday::Wed => Weekday::Thu,
      Weekday::Thu => Weekday::Fri,
      Weekday::Fri => Weekday::Sat,
      Weekday::Sat => Weekday::Sun,
      Weekday::Sun => Weekday::Mon,
    }
  }

  pub const fn predecessor(&self) -> Weekday {
    match *self {
      Weekday::Mon => Weekday::Sun,
      Weekday::Tue => Weekday::Mon,
      Weekday::Wed => Weekday::Tue,
      Weekday::Thu => Weekday::Wed,
      Weekday::Fri => Weekday::Thu,
      Weekday::Sat => Weekday::Fri,
      Weekday::Sun => Weekday::Sat,
    }
  }
  
  pub const fn number_of_days_since(&self, other: Weekday) -> u32 {
    let lhs = *self as u32;
    let rhs = other as u32;
    if lhs < rhs { 
      7 + lhs - rhs 
    } else { 
      lhs - rhs 
    }
  }

  pub const fn number_of_days_since_monday(&self) -> u32 {
    self.number_of_days_since(Weekday::Mon)
  }

  pub const fn number_of_days_since_sunday(&self) -> u32 {
    self.number_of_days_since(Weekday::Sun)
  }

  pub const fn as_number_from_monday(&self) -> u32 {
    self.number_of_days_since(Weekday::Mon) + 1
  }

  pub const fn as_number_from_sunday(&self) -> u32 {
    self.number_of_days_since(Weekday::Sun) + 1
  }
}

impl fmt::Display for Weekday {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.pad(match *self {
      Weekday::Mon => "Mon",
      Weekday::Tue => "Tue",
      Weekday::Wed => "Wed",
      Weekday::Thu => "Thu",
      Weekday::Fri => "Fri",
      Weekday::Sat => "Sat",
      Weekday::Sun => "Sun",
    })
  }
}
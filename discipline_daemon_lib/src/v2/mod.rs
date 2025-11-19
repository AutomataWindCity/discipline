use crate::x::{Countdown, CountdownConditional, Time, TimeRange, Weekday};

pub struct TimeConditional {
  mon: Option<TimeRange>,
  tue: Option<TimeRange>,
  wed: Option<TimeRange>,
  thu: Option<TimeRange>,
  fri: Option<TimeRange>,
  sat: Option<TimeRange>,
  sun: Option<TimeRange>,
}

impl TimeConditional {
  pub fn is_activated(&self, time: Time, weekday: Weekday) -> bool {
    match weekday {
      Weekday::Mon => {
        self.mon.is_some_and(|it| it.contains(time))
      }
      Weekday::Tue => {
        self.tue.is_some_and(|it| it.contains(time))
      }
      Weekday::Wed => {
        self.wed.is_some_and(|it| it.contains(time))
      }
      Weekday::Thu => {
        self.thu.is_some_and(|it| it.contains(time))
      }
      Weekday::Fri => {
        self.fri.is_some_and(|it| it.contains(time))
      }
      Weekday::Sat => {
        self.sat.is_some_and(|it| it.contains(time))
      }
      Weekday::Sun => {
        self.sun.is_some_and(|it| it.contains(time))
      }
    }
  }

  pub fn remove_sun(&mut self) {}
  pub fn remove_mon(&mut self) {}
  pub fn remove_tue(&mut self) {}
  pub fn remove_wed(&mut self) {}
  pub fn remove_thu(&mut self) {}
  pub fn remove_fri(&mut self) {}
  pub fn remove_sat(&mut self) {}
}

pub struct ScreenAccessRegulationProfile {
  allow_for: CountdownConditional,
  block_for: CountdownConditional,
  block_when: TimeConditional,
  // block_after_daily_allowance_is_up
  // block_after_weekly_allowance_is_up
}

pub struct InternetAccessRegulation {
  allow_for: CountdownConditional,
  block_for: CountdownConditional,
  block_when: TimeConditional,
}

// pub struct Permissions {
//   bitmap: i64,
// }

// impl Permissions {
//   const TIME_CONDITIONAL_REMOVE_MON: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_TUE: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_WED: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_THU: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_FRI: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_SAT: i64 = 0;
//   const TIME_CONDITIONAL_REMOVE_SUN: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_MON: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_TUE: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_WED: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_THU: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_FRI: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_SAT: i64 = 0;
//   const TIME_CONDITIONAL_CHANGE_SUN: i64 = 0;
// }

// pub struct Allower {
//   countdown: CountdownConditional,
// }
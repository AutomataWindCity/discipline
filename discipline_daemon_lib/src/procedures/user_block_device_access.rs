use serde::{Deserialize, Serialize};
use crate::x::{Duration, MonotonicInstant, ProfilesManager, TimeRange, UuidV4, WeekdaySet, block_device_access::ProfileName};

pub enum Procedure {
  BlockForTimeRangeAtWeekdays { 
    profile_id: UuidV4, 
    time_range: TimeRange,
    weekday_set: WeekdaySet,
  },
  BlockForDuration {
    duration: Duration,
  },
  ActivateProfile {
    profile_id: UuidV4,
  },
  DeactivateProfile {
    profile_id: UuidV4,
  },
  CreateProfile {
    profile_id: Option<UuidV4>,
    profile_name: ProfileName,
  },
  DeleteProfile {
    profile_id: UuidV4,
  },
}

pub enum ProcedureReturn {
  BlockInTimeRangeAtWeekdays,
  BlockForDuration,
  Activate,
  Deactivate,
  Create,
  Delete,
}

pub fn block_in_time_range_for_weekdays(
  
) {

}

// impl Procedure {
//   pub fn execute(
//     self, 
//   )
// }
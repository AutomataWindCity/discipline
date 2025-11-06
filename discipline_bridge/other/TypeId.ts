// // Fix the typos in your enum
// export const enum TypeId {
//   // ... existing
//   WeekdayTuesday, // was "Tuseday"
//   WeekdayWednesday, // was "Wednessday"
//   SerializationSerializer, // was "Serialiezr"
//   SerializationDeserializer, // was "Deserialiezr"
//   // ... rest
// }

export const enum TypeId {
  Countdown,
  DateTime,
  Time,
  Duration,
  Weekday,
  WeekdaySunday,
  WeekdayMonday,
  WeekdayTuseday,
  WeekdayWednessday,
  WeekdayThursday,
  WeekdayFriday,
  WeekdaySaturday,
  WeekdayRange,
  WeekdaySet,
  TimeRange,
  Rule,
  RuleGroup,
  RuleActivator,
  RuleProtector,
  CountdownConditional,
  CountdownAfterPleaConditional,
  CountdownAfterPleaConditionalStatusTypeEffective,
  CountdownAfterPleaConditionalStatusTypeGoingIneffective,
  CountdownAfterPleaConditionalStatusTypeIneffective,
  TimeRangeConditional,
  WeekdayRangeConditional,
  AlwaysConditional,
  TimeConditional,
  OptionNone,
  OptionSome,
  TriedSuccess,
  TriedFailure,
  SerializationSerialiezr,
  SerializationDeserialiezr,
  SerializationWriter,
  SerializationReader,
  UuidV4,
  RegulationBlockUserAccess,
  RegulationBlockDeviceAccess,
  RegulationBlockInternetAccess,
  RegulationBlockInfoAccess,
  OperatingSystemLinuxUserName,
  OperatingSystemLinuxUser,
  OperatingSystemLinuxPerUserInfo,
  OperatingSystemLinuxCrossUserInfo,
  UserGroup,
}

export interface Unique<Id extends TypeId = TypeId> {
  readonly typeId: Id
}
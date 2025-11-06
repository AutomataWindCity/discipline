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

export const withVirtualKey = <K extends string | number | symbol, T>(key: K, value: Omit<T, K>): T => {
  return value as T;
};

const TYPE_BRAND = Symbol();

export type Branded<Brand extends symbol, T> = (
  & T 
  & { [TYPE_BRAND]: Brand }
  & { [Key in Brand]: Brand }
);

export const Branded = <Brand extends symbol, T>(
  brand: Brand,
  value: Omit<T, Brand | typeof TYPE_BRAND>,
) => {
  return value as Branded<Brand, T>;
};
import { Time, TimeRange, Weekday, WeekdaySet, withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

export type TimeConditional = {
  readonly [BRAND]: "TImeConditional",
  readonly weekdays: WeekdaySet.WeekdaySet,
  readonly timeRange: TimeRange.TimeRange,
};

const construct = (
  weekdays: WeekdaySet.WeekdaySet,
  timeRange: TimeRange.TimeRange,
): TimeConditional => {
  return withVirtualKey(BRAND, {
    weekdays,
    timeRange,
  });
};

export const create = construct;

export const contains = (
  me: TimeConditional,
  weekday: Weekday.Weekday, 
  time: Time.Time,
): boolean => {
  return (
    WeekdaySet.contains(me.weekdays, weekday) 
    && 
    TimeRange.contains(me.timeRange, time)
  );
};
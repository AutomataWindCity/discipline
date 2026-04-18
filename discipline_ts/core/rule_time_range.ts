import { Branded, TimeRange, Countdown, Unique } from "../x.ts";

const BRAND = Symbol();

type RawTimeRangeRule = {
  condition: TimeRange,
  lifetime: Countdown,
};

export type TimeRangeRule = Unique<typeof BRAND, "TimeRangeRule", RawTimeRangeRule>;

const construct = (
  condition: TimeRange,
  lifetime: Countdown,
): TimeRangeRule => {
  return Branded(BRAND, {
    condition,
    lifetime,
  });
};

export const create = (
  timeRange: TimeRange,
  lifetime: Countdown,
): TimeRangeRule => {
  return construct(timeRange, lifetime);
};

export const getCondition = (it: TimeRangeRule): TimeRange => {
  return it.condition;
};

export const getLifetime = (it: TimeRangeRule): Countdown => {
  return it.lifetime;
};

export const TimeRangeRule = {
  construct,
  create,
  getCondition,
  getLifetime,
};
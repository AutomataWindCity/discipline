import { Branded, TimeRange, Countdown } from "../x.ts";

const BRAND = Symbol();

export type TimeRangeRule = Branded<typeof BRAND, {
  condition: TimeRange,
  lifetime: Countdown,
}>;

export const construct = (
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
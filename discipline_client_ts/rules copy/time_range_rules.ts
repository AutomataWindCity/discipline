import { TimeRangeRule, Nominal } from "../x.ts";

const BRAND = Symbol();

export type TimeRangeRules = Nominal<typeof BRAND, TimeRangeRule[]>;

export const construct = (rules: TimeRangeRule[]): TimeRangeRules => {
  return Nominal.create(BRAND, rules);
};

export const createDefault = (): TimeRangeRules => {
  return construct([]);
};

export const TimeRangeRules = {
  createDefault,
};
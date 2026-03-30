import { ScreenTimeAllowanceRule, Nominal } from "../x.ts";

const BRAND = Symbol();

export type TimeAllowanceRules = Nominal<typeof BRAND, ScreenTimeAllowanceRule[]>;

export const construct = (rules: ScreenTimeAllowanceRule[]): TimeAllowanceRules => {
  return Nominal.create(BRAND, rules);
};

export const createDefault = (): TimeAllowanceRules => {
  return construct([]);
};

export const TimeAllowanceRules = {
  createDefault,
};
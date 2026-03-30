import { Branded, CountdownRules, ScreenTimeAllowanceRules, TimeRangeRules } from "../x.ts";

const BRAND = Symbol();

export type ApplicationRule = Branded<typeof BRAND, {
  countdownRules: CountdownRules,
  timeRangeRules: TimeRangeRules,
  timeAllowanceRules: ScreenTimeAllowanceRules,
}>;

export const ApplicationRule = {

};
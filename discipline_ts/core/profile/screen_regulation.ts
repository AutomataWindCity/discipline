import { Branded, CountdownRules, ScreenTimeAllowanceRules, TimeRangeRules } from "../x.ts";

const BRAND = Symbol();

export type ScreenRegulation = Branded<typeof BRAND, {
  countdownRules: CountdownRules,
  timeRangeRules: TimeRangeRules,
  timeAllowanceRules: ScreenTimeAllowanceRules,
}>;

export const construct = (
  countdownRules: CountdownRules,
  timeRangeRules: TimeRangeRules,
  timeAllowanceRules: ScreenTimeAllowanceRules,
): ScreenRegulation => {
  return Branded(BRAND, {
    countdownRules,
    timeRangeRules,
    timeAllowanceRules,
  })
};

export const createDefault = () => {
  return construct(
    CountdownRules.createDefault(),
    TimeRangeRules.createDefault(),
    ScreenTimeAllowanceRules.createDefault(),
  )
}
export const ScreenRegulation = {
  createDefault,
};
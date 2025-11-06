import { withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

export type Weekday = number & {
  readonly [BRAND]: "Weekday",
};

export const MON_NUMBER = 0;
export const TUE_NUMBER = 1;
export const WED_NUMBER = 2;
export const THU_NUMBER = 3;
export const FRI_NUMBER = 4;
export const SAT_NUMBER = 5;
export const SUN_NUMBER = 6;

export const Mon = (): Weekday => {
  return withVirtualKey(BRAND, MON_NUMBER);
};
export const Tue = (): Weekday => {
  return withVirtualKey(BRAND, TUE_NUMBER);
};
export const Wed = (): Weekday => {
  return withVirtualKey(BRAND, WED_NUMBER);
};
export const Thu = (): Weekday => {
  return withVirtualKey(BRAND, THU_NUMBER);
};
export const Fri = (): Weekday => {
  return withVirtualKey(BRAND, FRI_NUMBER);
};
export const Sat = (): Weekday => {
  return withVirtualKey(BRAND, SAT_NUMBER);
};
export const Sun = (): Weekday => {
  return withVirtualKey(BRAND, SUN_NUMBER);
};

export interface WeekdayMatchCases<MonReturn, TueReturn, WedReturn, ThuReturn, FriReturn, SatReturn, SunReturn> {
  Mon: () => MonReturn,
  Tue: () => TueReturn,
  Wed: () => WedReturn,
  Thu: () => ThuReturn,
  Fri: () => FriReturn,
  Sat: () => SatReturn,
  Sun: () => SunReturn,
}

export const match = <A, B, C, D, E, F, J>(me: Weekday, cases: WeekdayMatchCases<A, B, C, D, E, F, J>) => {
  switch (me) {
    case MON_NUMBER: return cases.Mon();
    case TUE_NUMBER: return cases.Tue();
    case WED_NUMBER: return cases.Wed();
    case THU_NUMBER: return cases.Thu();
    case FRI_NUMBER: return cases.Fri();
    case SAT_NUMBER: return cases.Sat();
    case SUN_NUMBER: return cases.Sun();
    default: throw new Error("Matching against weekday number: Unreachable")
  } 
};
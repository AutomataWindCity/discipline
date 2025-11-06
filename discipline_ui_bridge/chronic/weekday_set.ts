import { Weekday, withVirtualKey } from "../mod.ts";

const MON = 0b0000001;
const TUE = 0b0000010;
const WED = 0b0000100;
const THU = 0b0001000;
const FRI = 0b0010000;
const SAT = 0b0100000;
const SUN = 0b1000000;
const ALL = 0b1111111;
const NONE = 0b0000000;

const BRAND = Symbol();

export type WeekdaySet = number & {
  readonly [BRAND]: "WeekdaySet",
};

const construct = (mask: number): WeekdaySet => {
  return withVirtualKey(BRAND, mask);
};

const mask = (me: WeekdaySet): number => {
  return me;
};

const bit = (weekday: Weekday.Weekday) => {
  return Weekday.match(weekday, {
    Mon: () => MON,
    Tue: () => TUE,
    Wed: () => WED,
    Fri: () => FRI,
    Sat: () => SAT,
    Thu: () => THU,
    Sun: () => SUN,
  });
};

export const createAllSet = (): WeekdaySet => {
  return construct(ALL);
};

export const createNoneSet = (): WeekdaySet => {
  return construct(NONE);
};

export const add = (me: WeekdaySet, weekday: Weekday.Weekday): WeekdaySet => {
  return construct(mask(me) | bit(weekday));
};

export const remove = (me: WeekdaySet, weekday: Weekday.Weekday): WeekdaySet => {
  return construct(mask(me) & ~bit(weekday));
};

export const contains = (me: WeekdaySet, weekday: Weekday.Weekday): boolean => {
  return !!(mask(me) & bit(weekday));
};

export const toggle = (me: WeekdaySet, weekday: Weekday.Weekday): WeekdaySet => {
  return construct(mask(me) ^ bit(weekday));
};
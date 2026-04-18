import { Integer, TextualError, Unique } from "../x.ts";

const BRAND = Symbol();

const MONDAY_AS_NUMBER = 0
const TUSEDAY_AS_NUMBER = 1
const WEDNESDAY_AS_NUMBER = 2
const THURSDAY_AS_NUMBER = 3
const FRIDAY_AS_NUMBER = 4
const SATURDAY_AS_NUMBER = 5
const SUNDAY_AS_NUMBER = 6

type RawMonday = typeof MONDAY_AS_NUMBER;
type RawTuesday = typeof TUSEDAY_AS_NUMBER;
type RawWednesday = typeof WEDNESDAY_AS_NUMBER;
type RawThursday = typeof THURSDAY_AS_NUMBER;
type RawFriday = typeof FRIDAY_AS_NUMBER;
type RawSaturday = typeof SATURDAY_AS_NUMBER;
type RawSunday = typeof SUNDAY_AS_NUMBER;

export type Monday = Unique<typeof BRAND, "Monday", typeof MONDAY_AS_NUMBER>;
export type Tuesday = Unique<typeof BRAND, "Tuesday", typeof TUSEDAY_AS_NUMBER>;
export type Wednesday = Unique<typeof BRAND, "Wednesday", typeof WEDNESDAY_AS_NUMBER>;
export type Thursday = Unique<typeof BRAND, "Thursday", typeof THURSDAY_AS_NUMBER>;
export type Friday = Unique<typeof BRAND, "Friday", typeof FRIDAY_AS_NUMBER>;
export type Saturday = Unique<typeof BRAND, "Saturday", typeof SATURDAY_AS_NUMBER>;
export type Sunday = Unique<typeof BRAND, "Sunday", typeof SUNDAY_AS_NUMBER>;

export const MONDAY = MONDAY_AS_NUMBER satisfies RawMonday as Monday;
export const TUSEDAY = TUSEDAY_AS_NUMBER satisfies RawTuesday as Tuesday;
export const WEDNESDAY = WEDNESDAY_AS_NUMBER satisfies RawWednesday as Wednesday;
export const THURSDAY = THURSDAY_AS_NUMBER satisfies RawThursday as Thursday;
export const FRIDAY = FRIDAY_AS_NUMBER satisfies RawFriday as Friday;
export const SATURDAY = SATURDAY_AS_NUMBER satisfies RawSaturday as Saturday;
export const SUNDAY = SUNDAY_AS_NUMBER satisfies RawSunday as Sunday;

export type Weekday = (
  | Monday
  | Tuesday
  | Wednesday
  | Thursday
  | Friday
  | Saturday
  | Sunday
);

/**
 * @throws {TextualError}
 */
const fromNumberOrThrow = (number: number): Weekday => {
  if (
    Number.isFinite(number)
    &&
    number >= MONDAY_AS_NUMBER
    &&
    number <= SUNDAY_AS_NUMBER
  ) {
    return number as Weekday;
  }

  const error = TextualError.create("Creating a Weekday from number");
  TextualError.addMessage(error, "Number must be a integer in this range: 0..=6 where 0 is Monday and 6 is Sunday");
  TextualError.addNumberAttachment(error, "Number", number);
  throw TextualError.toJsError(error);
};

const toNumber = (it: Weekday): number => {
  return it;
};

export const Weekday = {
  toNumber,
  fromNumberOrThrow,
}; 
import { Branded, Duration, FailureCode, TextualError, Tried } from "../x.ts";

const brand = Symbol();

export type Date = Branded<typeof brand, {
  date: globalThis.Date,
}>;

export const construct = (date: globalThis.Date): Date => {
  return Branded(brand, { date });
};

export const MINIMUM_TIMESTAMP = -8.64e15;
export const MAXIMUM_TIMESTAMP = 8.64e15;

export const now = (): Date => {
  const date = new globalThis.Date();
  date.setUTCHours(0, 0, 0, 0);
  return construct(date);
};

export const getTimestamp = (it: Date): number => {
  return it.date.getTime();
};

export const tillOrZero = (it: Date, rhs: Date): Duration => {
  const lhsTimestamp = it.date.getTime();
  const rhsTimestamp = rhs.date.getTime();

  if (lhsTimestamp < rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(rhsTimestamp - lhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const sinceOrZero = (it: Date, rhs: Date): Duration => {
  const lhsTimestamp = it.date.getTime();
  const rhsTimestamp = rhs.date.getTime();

  if (lhsTimestamp > rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(lhsTimestamp - rhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const fromTimestamp = (timestamp: number): Tried<Date, TextualError> => {
  if (!Number.isInteger(timestamp)) {
    const it = TextualError.create("Creating a Date from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    return Tried.Failure(it);
  }

  if (timestamp < MINIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a Date from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is less than the minimum value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Minimum value", MINIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }
  
  if (timestamp > MAXIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a Date from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is greater than the minimum value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Maximum value", MAXIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }

  const date = new globalThis.Date(timestamp);

  if (Number.isNaN(date.getTime())) {
    const it = TextualError.create("Creating a Date from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is valid, but failed to produce a valid JavaScript Date");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    return Tried.Failure(it);
  }

  if (
    date.getUTCHours() !== 0
    ||
    date.getUTCMinutes() !== 0
    ||
    date.getUTCSeconds() !== 0
    ||
    date.getUTCMilliseconds() !== 0
  ) {
    const it = TextualError.create("Creating a Date from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is valid, but produced a JavaScript Date with a non-zero time");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addStringAttachment(it, "JavaScript Date", date.toISOString());
    return Tried.Failure(it);
  }

  return Tried.Success(construct(date));
};

export const fromTimestampOrError = (
  timestamp: number,
  textualError: TextualError,
): Date | FailureCode => {

};

export const toString = (it: Date): string => {
  return it.date.toISOString();
};

export const getDayStart = (it: Date): Date => {
  const clone = new globalThis.Date(it.date);
  clone.setUTCHours(24, 0, 0, 0);
  return construct(clone);
};

export const getDurationTillMidnight = (it: Date): Duration => {
  const clone = new globalThis.Date(it.date);
  clone.setUTCHours(24, 0, 0, 0);

  const fromTimestamp = it.date.getTime();
  const tillTimestamp = clone.getTime();
  const difference = tillTimestamp - fromTimestamp;

  return Duration.fromMillisecondsOrThrow(difference);
};

export const isLaterThan = (it: Date, rhs: Date): boolean => {
  return getTimestamp(it) > getTimestamp(rhs);
};

export const Date = {
  MINIMUM_TIMESTAMP,
  MAXIMUM_TIMESTAMP,
  construct,
  now,
  getTimestamp,
  tillOrZero,
  sinceOrZero,
  fromTimestamp,
  fromTimestampOrError,
  toString,
  getDayStart,
  getDurationTillMidnight,
  isLaterThan,
};
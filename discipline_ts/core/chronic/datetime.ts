import { Date, Time, Duration, Branded, Tried, TextualError, Nominal, FailureCode } from "../x.ts";

const formatter = new Intl.DateTimeFormat("ar-SA", {
  year: "numeric",
  month: "numeric",
  day: "numeric",
  hour: "numeric",
  minute: "numeric",
  second: "numeric",
  hour12: true,
  calendar: "gregory",
  formatMatcher: "best fit",
});

const BRAND = Symbol();

export type DateTime = Nominal<typeof BRAND, globalThis.Date>;

export const MINIMUM_TIMESTAMP = -8.64e15;
export const MAXIMUM_TIMESTAMP = 8.64e15;

const isTimestampWithBounds = (timestamp: number) => {
  return timestamp >= MINIMUM_TIMESTAMP && timestamp <= MAXIMUM_TIMESTAMP;
}

export const construct = (date: globalThis.Date): DateTime => {
  return Nominal.create(BRAND, date);
};

export const now = (): DateTime => {
  return construct(new globalThis.Date());
};

export const fromTimestamp = (timestamp: number): Tried<DateTime, TextualError> => {
  if (!Number.isInteger(timestamp)) {
    const it = TextualError.create("Creating a DateTime from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    return Tried.Failure(it);
  }

  if (timestamp < MINIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a DateTime from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is less than the minimum valid value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Minimum valid value", MINIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }
  
  if (timestamp > MAXIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a DateTime from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is greater than the maximum valid value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Maximum valid value", MAXIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }

  const date = new globalThis.Date(timestamp);

  if (Number.isNaN(date.getTime())) {
    const it = TextualError.create("Creating a DateTime from a millisecond timestamp since the unix epoch");
    TextualError.addMessage(it, "Argument 'timestamp' is valid but didn't produce a valid JavaScript Date");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(date));
};

export const fromTimestampOrErrorCode = (
  timestamp: number,
  textualError: TextualError,
): DateTime | FailureCode => {

};

const getJsDate = (it: DateTime): globalThis.Date => {
  return Nominal.get(it);
};

export const getTime = (it: DateTime): Time => {
  const hour = getJsDate(it).getUTCHours();
  const minute = getJsDate(it).getUTCMinutes();
  return Tried.unwrap(Time.fromHourAndMinute(hour, minute));
};

export const toTimestamp = (it: DateTime): number => {
  return getJsDate(it).getTime();
};

export const tillOrZero = (lhs: DateTime, rhs: DateTime): Duration => {
  const lhsTimestamp = toTimestamp(lhs);
  const rhsTimestamp = toTimestamp(rhs);

  if (lhsTimestamp < rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(rhsTimestamp - lhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const sinceOrZero = (lhs: DateTime, rhs: DateTime): Duration => {
  const lhsTimestamp = toTimestamp(lhs);
  const rhsTimestamp = toTimestamp(rhs);

  if (lhsTimestamp > rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(lhsTimestamp - rhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const toString = (it: DateTime): string => {
  return getJsDate(it).toISOString();
};

export const getDate = (it: DateTime): Date => {
  const clone = new globalThis.Date(getJsDate(it));
  clone.setUTCHours(0, 0, 0, 0);
  return Date.construct(clone);
};

export const isAt = (lhs: DateTime, rhs: DateTime): boolean => {
  return toTimestamp(lhs) === toTimestamp(rhs);
};

export const isEarilerThan = (lhs: DateTime, rhs: DateTime): boolean => {
  return toTimestamp(lhs) < toTimestamp(rhs);
};

export const isEarilerThanOrAt = (lhs: DateTime, rhs: DateTime): boolean => {
  return toTimestamp(lhs) <= toTimestamp(rhs);
};

export const isLaterThan = (lhs: DateTime, rhs: DateTime): boolean => {
  return toTimestamp(lhs) > toTimestamp(rhs);
};

export const isLaterThanOrAt = (lhs: DateTime, rhs: DateTime): boolean => {
  return toTimestamp(lhs) >= toTimestamp(rhs);
};

export const isLaterThanBy = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isEqualTo(sinceOrZero(lhs, rhs), duration);
};

export const isLaterThanByOrLess = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isShorterThanOrEqualTo(sinceOrZero(lhs, rhs), duration);
};

export const isLaterThanByOrMore = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isLongerThanOrEqualTo(sinceOrZero(lhs, rhs), duration);
};

export const isEarilerThanBy = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isEqualTo(tillOrZero(lhs, rhs), duration);
};

export const isEarilerThanByOrLess = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isShorterThanOrEqualTo(tillOrZero(lhs, rhs), duration);
};

export const isEarilerThanByOrMore = (lhs: DateTime, rhs: DateTime, duration: Duration): boolean => {
  return Duration.isLongerThanOrEqualTo(tillOrZero(lhs, rhs), duration);
};

export const plusOrMax = (it: DateTime, duration: Duration) => {
  const timestamp = toTimestamp(it) + Duration.toTotalMilliseconds(duration);
  if (timestamp >= MAXIMUM_TIMESTAMP) {
    // TODO: This will NEVER produce an invalid date. But, just in case, 
    // do panic or somehow log an error if it does,
    const jsDate = new globalThis.Date(MAXIMUM_TIMESTAMP);
    return construct(jsDate);
  }
  
  return construct(new globalThis.Date(timestamp));
};

export const toString2 = (it: DateTime): string => {
  return formatter.format(toTimestamp(it));
};

export const DateTime = {
  MINIMUM_TIMESTAMP,
  MAXIMUM_TIMESTAMP,
  now,
  getTime,
  toTimestamp,
  tillOrZero,
  sinceOrZero,
  fromTimestamp,
  fromTimestampOrErrorCode,
  toString,
  getDate,
  isAt,
  isEarilerThan,
  isEarilerThanOrAt,
  isLaterThan,
  isLaterThanOrAt,
  plusOrMax,
  toString2,
  isEarilerThanBy,
  isEarilerThanByOrLess,
  isEarilerThanByOrMore,
  isLaterThanBy,
  isLaterThanByOrLess,
  isLaterThanByOrMore,
};
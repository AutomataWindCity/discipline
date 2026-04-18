import { FAILURE, FailureCode, TextualError, Tried, Unique } from "../x.ts";

const BRAND = Symbol();

export type Duration = Unique<typeof BRAND, "Duration", number>;

const construct = (milliseconds: number): Duration => {
  return milliseconds as Duration;
};

const MAXIMUM_MILLISECONDS = Number.MAX_SAFE_INTEGER;
const MAXIMUM_SECONDS = Math.floor(MAXIMUM_MILLISECONDS / 1000);
const MAXIMUM_MINUTES = Math.floor(MAXIMUM_SECONDS / 60);
const MAXIMUM_HOURS = Math.floor(MAXIMUM_MINUTES / 60);
const MAXIMUM_DAYS = Math.floor(MAXIMUM_HOURS / 24);
const MAXIMUM_WEEKS = Math.floor(MAXIMUM_DAYS / 7);

const MILLISECONDS_PER_SECOND = 1000;
const MILLISECONDS_PER_MINUTE = MILLISECONDS_PER_SECOND * 60;
const MILLISECONDS_PER_HOUR = MILLISECONDS_PER_MINUTE * 60;
const MILLISECONDS_PER_DAY = MILLISECONDS_PER_HOUR * 24;
const MILLISECONDS_PER_WEEK = MILLISECONDS_PER_DAY * 7;

const DAY = construct(MILLISECONDS_PER_DAY);

const fromMilliseconds = (milliseconds: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(milliseconds)) {
    const it = TextualError.create("Creating a Duration from milliseconds");
    TextualError.addMessage(it, "Argument 'milliseconds' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'milliseconds'", milliseconds);
    return Tried.Failure(it);
  }
  
  if (milliseconds < 0) {
    const it = TextualError.create("Creating a Duration from milliseconds");
    TextualError.addMessage(it, "Argument 'milliseconds' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'milliseconds'", milliseconds);
    return Tried.Failure(it);
  }
  
  if (milliseconds > MAXIMUM_MILLISECONDS) {
    const it = TextualError.create("Creating a Duration from milliseconds");
    TextualError.addMessage(it, "Argument 'milliseconds' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'milliseconds'", milliseconds);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_MILLISECONDS);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(milliseconds));
};

const fromMillisecondsOrErrorCode = (
  milliseconds: number,
  textualError: TextualError,
): Duration | FailureCode => {
  if (!Number.isInteger(milliseconds)) {
    TextualError.changeContext(textualError, "Creating a Duration from milliseconds");
    TextualError.addMessage(textualError, "Argument 'milliseconds' is not an integer")
    TextualError.addNumberAttachment(textualError, "Argument 'milliseconds'", milliseconds);
    return FAILURE;
  }
  
  if (milliseconds < 0) {
    TextualError.changeContext(textualError, "Creating a Duration from milliseconds");
    TextualError.addMessage(textualError, "Argument 'milliseconds' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(textualError, "Argument 'milliseconds'", milliseconds);
    return FAILURE;
  }
  
  if (milliseconds > MAXIMUM_MILLISECONDS) {
    TextualError.changeContext(textualError, "Creating a Duration from milliseconds");
    TextualError.addMessage(textualError, "Argument 'milliseconds' is greater than maximum value")
    TextualError.addNumberAttachment(textualError, "Argument 'milliseconds'", milliseconds);
    TextualError.addNumberAttachment(textualError, "Maximum value",  MAXIMUM_MILLISECONDS);
    return FAILURE;
  }

  return construct(milliseconds);
};

const fromSeconds = (seconds: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(seconds)) {
    const it = TextualError.create("Creating a Duration from seconds");
    TextualError.addMessage(it, "Argument 'seconds' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'seconds'", seconds);
    return Tried.Failure(it);
  }
  
  if (seconds < 0) {
    const it = TextualError.create("Creating a Duration from seconds");
    TextualError.addMessage(it, "Argument 'seconds' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'seconds'", seconds);
    return Tried.Failure(it);
  }
  
  if (seconds > MAXIMUM_SECONDS) {
    const it = TextualError.create("Creating a Duration from seconds");
    TextualError.addMessage(it, "Argument 'seconds' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'seconds'", seconds);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_SECONDS);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(seconds * MILLISECONDS_PER_SECOND));
};

const fromMinutes = (minutes: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(minutes)) {
    const it = TextualError.create("Creating a Duration from minutes");
    TextualError.addMessage(it, "Argument 'minutes' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'minutes'", minutes);
    return Tried.Failure(it);
  }
  
  if (minutes < 0) {
    const it = TextualError.create("Creating a Duration from minutes");
    TextualError.addMessage(it, "Argument 'minutes' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'minutes'", minutes);
    return Tried.Failure(it);
  }
  
  if (minutes > MAXIMUM_MINUTES) {
    const it = TextualError.create("Creating a Duration from minutes");
    TextualError.addMessage(it, "Argument 'minutes' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'minutes'", minutes);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_MINUTES);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(minutes * MILLISECONDS_PER_MINUTE));
};

const fromHours = (hours: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(hours)) {
    const it = TextualError.create("Creating a Duration from hours");
    TextualError.addMessage(it, "Argument 'hours' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'hours'", hours);
    return Tried.Failure(it);
  }
  
  if (hours < 0) {
    const it = TextualError.create("Creating a Duration from hours");
    TextualError.addMessage(it, "Argument 'hours' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'hours'", hours);
    return Tried.Failure(it);
  }
  
  if (hours > MAXIMUM_HOURS) {
    const it = TextualError.create("Creating a Duration from hours");
    TextualError.addMessage(it, "Argument 'hours' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'hours'", hours);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_HOURS);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(hours * MILLISECONDS_PER_HOUR));
};

const fromDays = (days: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(days)) {
    const it = TextualError.create("Creating a Duration from days");
    TextualError.addMessage(it, "Argument 'days' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'days'", days);
    return Tried.Failure(it);
  }
  
  if (days < 0) {
    const it = TextualError.create("Creating a Duration from days");
    TextualError.addMessage(it, "Argument 'days' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'days'", days);
    return Tried.Failure(it);
  }
  
  if (days > MAXIMUM_DAYS) {
    const it = TextualError.create("Creating a Duration from days");
    TextualError.addMessage(it, "Argument 'days' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'days'", days);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_DAYS);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(days * MILLISECONDS_PER_DAY));
};

const fromWeeks = (weeks: number): Tried<Duration, TextualError> => {
  if (!Number.isInteger(weeks)) {
    const it = TextualError.create("Creating a Duration from weeks");
    TextualError.addMessage(it, "Argument 'weeks' is not an integer")
    TextualError.addNumberAttachment(it, "Argument 'weeks'", weeks);
    return Tried.Failure(it);
  }
  
  if (weeks < 0) {
    const it = TextualError.create("Creating a Duration from weeks");
    TextualError.addMessage(it, "Argument 'weeks' is negative: This Duration type only suuports representing positive durations")
    TextualError.addNumberAttachment(it, "Argument 'weeks'", weeks);
    return Tried.Failure(it);
  }
  
  if (weeks > MAXIMUM_WEEKS) {
    const it = TextualError.create("Creating a Duration from weeks");
    TextualError.addMessage(it, "Argument 'weeks' is greater than maximum value")
    TextualError.addNumberAttachment(it, "Argument 'weeks'", weeks);
    TextualError.addNumberAttachment(it, "Maximum value",  MAXIMUM_WEEKS);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(weeks * MILLISECONDS_PER_WEEK));
};

const fromMillisecondsOrThrow = (millseconds: number): Duration => {
  const it = fromMilliseconds(millseconds);
  if (Tried.isSuccess(it)) {
    return Tried.value(it);
  } else {
    throw new Error(TextualError.prettyPrint(Tried.error(it)));
  }
};

const zero = (): Duration => {
  return construct(0);
};

const isZero = (it: Duration): boolean => {
  return toTotalMilliseconds(it) === 0;
};

const toTotalMilliseconds = (it: Duration): number => {
  return it;
};

const toTotaMinutes = (it: Duration): number => {
  return Math.floor(toTotalMilliseconds(it) / MILLISECONDS_PER_MINUTE);
};

const minusOrZero = (it: Duration, rhs: Duration): Duration => {
  if (toTotalMilliseconds(it) > toTotalMilliseconds(rhs)) {
    return construct(toTotalMilliseconds(it) - toTotalMilliseconds(rhs));
  } else {
    return zero();
  }
};

const plusOrMax = (it: Duration, rhs: Duration): Duration => {
  const result = toTotalMilliseconds(it) + toTotalMilliseconds(rhs);
  if (result <= MAXIMUM_MILLISECONDS) {
    return construct(result);
  } else {
    return construct(MAXIMUM_MILLISECONDS);
  }
};

const isEqualTo = (lhs: Duration, rhs: Duration): boolean => {
  return toTotalMilliseconds(lhs) === toTotalMilliseconds(rhs);
};

const isLongerThan = (it: Duration, rhs: Duration): boolean => {
  return toTotalMilliseconds(it) > toTotalMilliseconds(rhs);
};

const isLongerThanOrEqualTo = (it: Duration, rhs: Duration): boolean => {
  return toTotalMilliseconds(it) >= toTotalMilliseconds(rhs);
};

const isShorterThan = (it: Duration, rhs: Duration): boolean => {
  return toTotalMilliseconds(it) < toTotalMilliseconds(rhs);
};

const isShorterThanOrEqualTo = (it: Duration, rhs: Duration): boolean => {
  return toTotalMilliseconds(it) <= toTotalMilliseconds(rhs);
};

const min = (lhs: Duration, rhs: Duration): Duration => {
  return isShorterThan(lhs, rhs) ? lhs : rhs;
};

const max = (lhs: Duration, rhs: Duration): Duration => {
  return isLongerThan(lhs, rhs) ? lhs : rhs;
};

const toString = (it: Duration): string => {
  const parts: string[] = [];

  let totalMilliseconds = toTotalMilliseconds(it);
  
  const totalDays = Math.floor(totalMilliseconds / MILLISECONDS_PER_DAY);
  totalMilliseconds %= MILLISECONDS_PER_DAY;
  parts.push(`${totalDays} D`);
  
  const totalHours = Math.floor(totalMilliseconds / MILLISECONDS_PER_HOUR);
  totalMilliseconds %= MILLISECONDS_PER_HOUR;
  parts.push(`${totalHours} H`);
  
  const totalMinutes = Math.floor(totalMilliseconds / MILLISECONDS_PER_MINUTE);
  totalMilliseconds %= MILLISECONDS_PER_MINUTE;
  parts.push(`${totalMinutes} M`);
  
  return parts.join(' ');
};

const toString2 = (it: Duration): string => {
  const milliseconds = toTotalMilliseconds(it);

  if (milliseconds === 0) {
    return '0s';
  }

  const totalSeconds = Math.floor(milliseconds / 1000);
  const totalMinutes = Math.floor(totalSeconds / 60);
  const totalHours = Math.floor(totalMinutes / 60);
  const totalDays = Math.floor(totalHours / 24);

  const seconds = totalSeconds % 60;
  const minutes = totalMinutes % 60;
  const hours = totalHours % 24;
  const days = totalDays;

  const parts: string[] = [];

  if (days > 0) {
    parts.push(`${days}d`);
  }
  if (hours > 0) {
    parts.push(`${hours}h`);
  }
  if (minutes > 0) {
    parts.push(`${minutes}m`);
  }
  if (seconds > 0 || parts.length === 0) {
    parts.push(`${seconds}s`);
  }

  return parts.join(' ');
};

export const Duration = {
  MAXIMUM_MILLISECONDS,
  MAXIMUM_SECONDS,
  MAXIMUM_MINUTES,
  MAXIMUM_HOURS,
  MILLISECONDS_PER_SECOND,
  MILLISECONDS_PER_MINUTE,
  MILLISECONDS_PER_HOUR,
  MILLISECONDS_PER_DAY,
  MILLISECONDS_PER_WEEK,
  DAY,
  fromMilliseconds,
  fromMillisecondsOrErrorCode,
  fromMinutes,
  fromSeconds,
  fromHours,
  fromDays,
  fromWeeks,
  zero,
  toTotalMilliseconds,
  toTotaMinutes,
  isZero,
  saturatingSub: minusOrZero,
  saturatingAdd: plusOrMax,
  isEqualTo,
  isLongerThan,
  isLongerThanOrEqualTo,
  isShorterThan,
  isShorterThanOrEqualTo,
  min,
  max,
  toString,
  toString2,
  fromMillisecondsOrThrow,
};
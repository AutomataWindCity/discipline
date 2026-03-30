import { Duration, TextualError, Nominal, Tried, FailureCode } from "../x.ts";

const brand = Symbol();

export type Time = Nominal<typeof brand, number>;

export const construct = (timestamp: number): Time => {
  return Nominal.create(brand, timestamp);
};

export const MINIMUM_TIMESTAMP = 0;
export const MAXIMUM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1;

export const fromTimestamp = (timestamp: number): Tried<Time, TextualError> => {
  if (!Number.isInteger(timestamp)) {
    const it = TextualError.create("Creating a Time from a millisecond timestamp since midnight");
    TextualError.addMessage(it, "Argument 'timestamp' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    return Tried.Failure(it);
  }

  if (timestamp < MINIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a Time from a millisecond timestamp since midnight");
    TextualError.addMessage(it, "Argument 'timestamp' is less than the minimum valid value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Minimum valid value", MINIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }

  if (timestamp > MINIMUM_TIMESTAMP) {
    const it = TextualError.create("Creating a Time from a millisecond timestamp since midnight");
    TextualError.addMessage(it, "Argument 'timestamp' is greater than the maximum valid value");
    TextualError.addNumberAttachment(it, "Argument 'timestamp'", timestamp);
    TextualError.addNumberAttachment(it, "Maximum valid value", MAXIMUM_TIMESTAMP);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(timestamp));
};

export const fromTimestampOrError = (
  timestamp: number,
  textualError: TextualError,
): Time | FailureCode => {

};

export const fromHourAndMinuteAm = (hour: number, minute: number): Tried<Time, TextualError> => {
  if (!Number.isInteger(hour)) {
    const it = TextualError.create("Creating a Time from a hour (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour < 0) {
    const it = TextualError.create("Creating a Time from a hour (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour > 11) {
    const it = TextualError.create("Creating a Time from a hour (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is greater than '11'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }

  if (!Number.isInteger(minute)) {
    const it = TextualError.create("Creating a Time from a minute (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute < 0) {
    const it = TextualError.create("Creating a Time from a minute (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute > 59) {
    const it = TextualError.create("Creating a Time from a minute (AM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is greater than '59'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(
    hour * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  ));
};

export const fromHourAndMinutePm = (hour: number, minute: number): Tried<Time, TextualError> => {
  if (!Number.isInteger(hour)) {
    const it = TextualError.create("Creating a Time from a hour (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour < 0) {
    const it = TextualError.create("Creating a Time from a hour (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour > 11) {
    const it = TextualError.create("Creating a Time from a hour (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is greater than '11'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }

  if (!Number.isInteger(minute)) {
    const it = TextualError.create("Creating a Time from a minute (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute < 0) {
    const it = TextualError.create("Creating a Time from a minute (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute > 59) {
    const it = TextualError.create("Creating a Time from a minute (PM) and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is greater than '59'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(
    (12 + hour) * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  ));
};

export const fromHourAndMinute = (hour: number, minute: number): Tried<Time, TextualError> => {
  if (!Number.isInteger(hour)) {
    const it = TextualError.create("Creating a Time from a hour and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour < 0) {
    const it = TextualError.create("Creating a Time from a hour and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }
  if (hour > 23) {
    const it = TextualError.create("Creating a Time from a hour and minute arguments");
    TextualError.addMessage(it, "Argument 'hour' is greater than '23'");
    TextualError.addNumberAttachment(it, "Argument 'hour'", hour);
    return Tried.Failure(it);
  }

  if (!Number.isInteger(minute)) {
    const it = TextualError.create("Creating a Time from a minute and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute < 0) {
    const it = TextualError.create("Creating a Time from a minute and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is less than '0'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }
  if (minute > 59) {
    const it = TextualError.create("Creating a Time from a minute and minute arguments");
    TextualError.addMessage(it, "Argument 'minute' is greater than '59'");
    TextualError.addNumberAttachment(it, "Argument 'minute'", minute);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(
    hour * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  ));
};

export const getTimestamp = (it: Time): number => {
  return Nominal.get(it);
};

export const getHour = (it: Time): number => {
  return Math.floor(
    getTimestamp(it) 
    /
    Duration.MILLISECONDS_PER_HOUR
  );
};

export const getMinute = (it: Time): number => {
  return Math.floor(
    getTimestamp(it) 
    % 
    Duration.MILLISECONDS_PER_HOUR 
    / 
    Duration.MILLISECONDS_PER_MINUTE
  );
};

export const getSecond = (it: Time): number => {
  return Math.floor(
    getTimestamp(it)
    % 
    Duration.MILLISECONDS_PER_HOUR 
    %
    Duration.MILLISECONDS_PER_MINUTE
    /
    Duration.MILLISECONDS_PER_SECOND
  );
};

export const toString = (it: Time): string => {
  return `${
    getHour(it).toString()
  }:${
    getMinute(it).toString()
  }:${
    getSecond(it).toString()
  }`;
};

export const Time = {
  MINIMUM_TIMESTAMP,
  MAXIMUM_TIMESTAMP,
  fromTimestamp,
  fromTimestampOrError,
  fromHourAndMinute,
  fromHourAndMinuteAm,
  fromHourAndMinutePm,
  getTimestamp,
  getHour,
  getMinute,
  getSecond,
  toString,
};
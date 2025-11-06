import { Duration, Option, withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

export type Time = number & {
  readonly [BRAND]: "Time",
};

export const MINIMUM_TIMESTAMP = 0;
export const MAXIMUM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1;

const construct = (inner: number): Time => {
  return withVirtualKey(BRAND, inner);
};

export const fromTimestampOrNone = (timestamp: number): Option.Option<Time> => {
  if (
    Number.isInteger(timestamp) 
    && 
    timestamp >= MINIMUM_TIMESTAMP
    &&
    timestamp <= MAXIMUM_TIMESTAMP
  ) {
    return Option.Some(construct(timestamp));
  }

  return Option.None();
};

export const fromHourAndMinuteAmOrThrow = (hour: number, minute: number): Time => {
  if (!Number.isInteger(hour)) {
    throw new Error("Creating Time from hour (AM) and minute: Hour is not integer");
  }
  if (hour < 0) {
    throw new Error("Creating Time from hour (AM) and minute: Hour is less than zero");
  }
  if (hour > 11) {
    throw new Error("Creating Time from hour (AM) and minute: Hour is greater than 11");
  }
  if (!Number.isInteger(minute)) {
    throw new Error("Creating Time from hour (AM) and minute: Minute is not integer");
  }
  if (minute < 0) {
    throw new Error("Creating Time from hour (AM) and minute: Minute less than zero");
  }
  if (minute > 59) {
    throw new Error("Creating Time from hour (AM) and minute: Minute is greater than 59");
  }

  return construct(
    hour * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  )
};

export const fromHourAndMinutePmOrThrow = (hour: number, minute: number): Time => {
  if (!Number.isInteger(hour)) {
    throw new Error("Creating Time from hour (PM) and minute: Hour is not integer");
  }
  if (hour < 0) {
    throw new Error("Creating Time from hour (PM) and minute: Hour is less than zero");
  }
  if (hour > 11) {
    throw new Error("Creating Time from hour (PM) and minute: Hour is greater than 11");
  }
  if (!Number.isInteger(minute)) {
    throw new Error("Creating Time from hour (PM) and minute: Minute is not integer");
  }
  if (minute < 0) {
    throw new Error("Creating Time from hour (PM) and minute: Minute less than zero");
  }
  if (minute > 59) {
    throw new Error("Creating Time from hour (PM) and minute: Minute is greater than 59");
  }

  return construct(
    (12 + hour) * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  );
};

export const fromHourAndMinuteOrThrow = (hour: number, minute: number): Time => {
  if (!Number.isInteger(hour)) {
    throw new Error("Creating Time from hour and minute: Hour is not integer");
  }
  if (hour < 0) {
    throw new Error("Creating Time from hour and minute: Hour is less than zero");
  }
  if (hour > 23) {
    throw new Error("Creating Time from hour and minute: Hour is greater than 23");
  }
  if (!Number.isInteger(minute)) {
    throw new Error("Creating Time from hour and minute: Minute is not integer");
  }
  if (minute < 0) {
    throw new Error("Creating Time from hour and minute: Minute less than zero");
  }
  if (minute > 59) {
    throw new Error("Creating Time from hour and minute: Minute is greater than 59");
  }

  return construct(
    hour * Duration.MILLISECONDS_PER_HOUR
    +
    minute * Duration.MILLISECONDS_PER_MINUTE
  );
};

export const timestamp = (me: Time): number => {
  return me;
};
import { Tried, withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

export type Duration = number & {
  readonly [BRAND]: "Duration"
};

export const MAXIMUM_MILLISECONDS = Number.MAX_SAFE_INTEGER;
export const MAXIMUM_SECONDS = MAXIMUM_MILLISECONDS / 1000;
export const MAXIMUM_MINUTES = MAXIMUM_SECONDS / 60;
export const MAXIMUM_HOURS = MAXIMUM_MINUTES / 60;

export const MILLISECONDS_PER_SECOND = 1000;
export const MILLISECONDS_PER_MINUTE = MILLISECONDS_PER_SECOND * 60;
export const MILLISECONDS_PER_HOUR = MILLISECONDS_PER_MINUTE * 60;
export const MILLISECONDS_PER_DAY = MILLISECONDS_PER_HOUR * 24;
export const MILLISECONDS_PER_WEEK = MILLISECONDS_PER_DAY * 7;

const construct = (inner: number): Duration => {
  return withVirtualKey(BRAND, inner);
};

export const fromMilliseconds = (milliseconds: number): Tried.Tried<Duration, Error> => {
  if (Number.isInteger(milliseconds)) {
    return Tried.Failure(new Error("Creating Duration from milliseconds: Argument 'milliseconds' is integer"));
  }
  if (milliseconds < 0) {
    return Tried.Failure(new Error("Creating Duration from milliseconds: Argument 'milliseconds' is less than minium value which is zero"));
  }
  if (milliseconds > MAXIMUM_MILLISECONDS) {
    return Tried.Failure(new Error(`Creating Duration from milliseconds: Argument 'milliseconds' is greater than maximum value which is ${MAXIMUM_MILLISECONDS}`));
  }
  return Tried.Success(construct(milliseconds));
};

export const fromMillisecondsOrThrow = (milliseconds: number): Duration => {
  if (Number.isInteger(milliseconds)) {
    throw new Error("Creating Duration from milliseconds: Argument 'milliseconds' is integer");
  }
  if (milliseconds < 0) {
    throw new Error("Creating Duration from milliseconds: Argument 'milliseconds' is less than minium value which is zero");
  }
  if (milliseconds > MAXIMUM_MILLISECONDS) {
    throw new Error(`Creating Duration from milliseconds: Argument 'milliseconds' is greater than maximum value which is ${MAXIMUM_MILLISECONDS}`);
  }
  return construct(milliseconds);
}

export const fromHoursOrThrow = (hours: number): Duration => {
  if (Number.isInteger(hours)) {
    throw new Error("Creating Duration from hours: Argument 'hours' is integer");
  }
  if (hours < 0) {
    throw new Error("Creating Duration from hours: Argument 'hours' is less than minium value which is zero");
  }
  if (hours > MAXIMUM_HOURS) {
    throw new Error(`Creating Duration from hours: Argument 'hours' is greater than maximum value which is ${MAXIMUM_HOURS}`);
  }
  return construct(hours * MILLISECONDS_PER_HOUR);
}

export const zero = (): Duration => {
  return construct(0);
};

export const isZero = (me: Duration): boolean => {
  return milliseconds(me) === 0;
};

export const milliseconds = (me: Duration): number => {
  return me;
};

export const minusOrZero = (lhs: Duration, rhs: Duration): Duration => {
  if (milliseconds(lhs) > milliseconds(rhs)) {
    return construct(milliseconds(lhs) - milliseconds(rhs));
  } else {
    return zero();
  }
};
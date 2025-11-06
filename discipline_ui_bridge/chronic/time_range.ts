import { Duration, Option, Time, withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

export type TimeRange = {
  readonly from: number,
  readonly till: number,
  readonly [BRAND]: "TimeRange",
};

const construct = (from: number, till: number): TimeRange => {
  return withVirtualKey(BRAND, {
    from,
    till,
  });
};

const MINIMUM_FROM_TIMESTAMP = 0;
const MAXIMUM_FROM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1;

const MINIMUM_TILL_TIMESTAMP = 0;
const MAXIMUM_TILL_TIMESTAMP = 1000 * 60 * 60 * 24 * 2 - 1;

export const fromHour12AndMinuteOrNone = (from: number, till: number): Option.Option<TimeRange> => {
  if (
    Number.isInteger(from)
    &&
    from >= MINIMUM_FROM_TIMESTAMP
    &&
    from <= MAXIMUM_FROM_TIMESTAMP
    &&
    Number.isInteger(from)
    &&
    till >= MINIMUM_TILL_TIMESTAMP
    &&
    till <= MAXIMUM_TILL_TIMESTAMP
    &&
    from <= till
    &&
    (till - from) <= Duration.MILLISECONDS_PER_DAY
  ) {
    return Option.Some(construct(from, till));
  }

  return Option.None();
};

export const fromTimes = (from: Time.Time, till: Time.Time): TimeRange => {
  const fromTimestamp = Time.timestamp(from);
  const tillTimestamp = Time.timestamp(till);

  if (fromTimestamp <= tillTimestamp) {
    return construct(fromTimestamp, tillTimestamp);
  } else {
    return construct(fromTimestamp, tillTimestamp + Duration.MILLISECONDS_PER_DAY);
  }
}

export const contains = (me: TimeRange, time: Time.Time): boolean => {
  return (
    Time.timestamp(time) >= me.from 
    && 
    Time.timestamp(time) <= me.till
  );
};